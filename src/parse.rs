//! Read a PCZT or a signed transaction with librustzcash. No hand-written bundle parser.

use zcash_primitives::transaction::{Transaction, TxVersion};
use zcash_protocol::{
    consensus::BranchId,
    value::{ZatBalance, Zatoshis},
};

use crate::policy::{inspect, Exposure, Policy, Verdict};

/// Nu6 is only the v4 hint. v5 and v6 carry their own branch id in the header.
const V4_BRANCH_HINT: BranchId = BranchId::Nu6;

pub fn inspect_hex(text: &str, policy: Policy) -> Result<Verdict, String> {
    let bytes = decode_hex(text)?;
    inspect_bytes(&bytes, policy)
}

pub fn inspect_bytes(bytes: &[u8], policy: Policy) -> Result<Verdict, String> {
    if bytes.len() >= 4 && &bytes[..4] == b"PCZT" {
        return inspect_pczt(bytes, policy);
    }
    inspect_transaction(bytes, policy)
}

fn inspect_transaction(bytes: &[u8], policy: Policy) -> Result<Verdict, String> {
    let mut cursor = std::io::Cursor::new(bytes);
    match TxVersion::read(&mut cursor) {
        Ok(_) => {}
        Err(err) if is_unknown_format(&err) => {
            let mut exposure = Exposure::empty_shielded();
            exposure.recognized = false;
            return Ok(inspect(&exposure, policy));
        }
        Err(err) => return Err(format!("unreadable transaction: {err}")),
    }

    let tx = match Transaction::read(std::io::Cursor::new(bytes), V4_BRANCH_HINT) {
        Ok(tx) => tx,
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
            return Err(format!("unreadable transaction: {err}"));
        }
        Err(_) => {
            let mut exposure = Exposure::empty_shielded();
            exposure.recognized = false;
            return Ok(inspect(&exposure, policy));
        }
    };

    let mut exposure = Exposure::empty_shielded();
    if let Some(bundle) = tx.transparent_bundle() {
        exposure.transparent_input_count = u32::try_from(bundle.vin.len()).unwrap_or(u32::MAX);
        exposure.transparent_output_zatoshis = bundle
            .vout
            .iter()
            .map(|output| zatoshis_i64(output.value()))
            .collect();
    }
    if let Some(bundle) = tx.sprout_bundle() {
        match bundle.value_balance() {
            Some(balance) => exposure.sprout_zatoshis = zat_i64(balance),
            None => exposure.recognized = false,
        }
    }
    exposure.sapling_zatoshis = zat_i64(tx.sapling_value_balance());
    if let Some(bundle) = tx.orchard_bundle() {
        exposure.orchard_zatoshis = zat_i64(*bundle.value_balance());
    }
    if let Some(bundle) = tx.ironwood_bundle() {
        exposure.ironwood_zatoshis = zat_i64(*bundle.value_balance());
    }
    Ok(inspect(&exposure, policy))
}

fn inspect_pczt(bytes: &[u8], policy: Policy) -> Result<Verdict, String> {
    let pczt = pczt::parse(bytes).map_err(|err| format!("unreadable pczt: {err}"))?;
    let mut exposure = Exposure::empty_shielded();

    let inputs = pczt.transparent().inputs();
    exposure.transparent_input_count = u32::try_from(inputs.len()).unwrap_or(u32::MAX);
    if !inputs.is_empty() {
        let mut sum: i64 = 0;
        for input in inputs {
            let value = i64::try_from(*input.value()).map_err(|_| "pczt input value overflows")?;
            sum = sum.checked_add(value).ok_or("pczt input sum overflows")?;
            if !input.proprietary().is_empty() {
                exposure.pczt_metadata = true;
            }
        }
        exposure.transparent_input_zatoshis = Some(sum);
    }
    for output in pczt.transparent().outputs() {
        let value = i64::try_from(*output.value()).map_err(|_| "pczt output value overflows")?;
        exposure.transparent_output_zatoshis.push(value);
        if output.user_address().is_some() || !output.proprietary().is_empty() {
            exposure.pczt_metadata = true;
        }
    }

    exposure.sapling_zatoshis =
        i64::try_from(*pczt.sapling().value_sum()).map_err(|_| "sapling value_sum overflows")?;
    exposure.orchard_zatoshis = signed_magnitude(*pczt.orchard().value_sum())?;
    exposure.ironwood_zatoshis = signed_magnitude(*pczt.ironwood().value_sum())?;

    for spend in pczt.sapling().spends() {
        if spend.witness().is_some() || !spend.proprietary().is_empty() {
            exposure.pczt_metadata = true;
        }
    }
    for output in pczt.sapling().outputs() {
        if output.user_address().is_some() || !output.proprietary().is_empty() {
            exposure.pczt_metadata = true;
        }
    }

    Ok(inspect(&exposure, policy))
}

fn signed_magnitude(value: (u64, bool)) -> Result<i64, String> {
    let (magnitude, negative) = value;
    let magnitude = i64::try_from(magnitude).map_err(|_| "pool value overflows")?;
    Ok(if negative { -magnitude } else { magnitude })
}

fn zat_i64(value: ZatBalance) -> i64 {
    i64::from_le_bytes(value.to_i64_le_bytes())
}

fn zatoshis_i64(value: Zatoshis) -> i64 {
    zat_i64(ZatBalance::from(value))
}

fn is_unknown_format(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::InvalidData && err.to_string().contains("Unknown transaction")
}

fn decode_hex(text: &str) -> Result<Vec<u8>, String> {
    let mut cleaned = String::new();
    for ch in text.chars() {
        if ch.is_ascii_hexdigit() {
            cleaned.push(ch);
        } else if ch.is_whitespace() || ch == 'x' || ch == 'X' {
            continue;
        } else {
            return Err(format!("invalid hex character {ch}"));
        }
    }
    if cleaned.len() % 2 != 0 {
        return Err("hex has an odd length".into());
    }
    if cleaned.is_empty() {
        return Err("empty input".into());
    }
    hex::decode(&cleaned).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{Code, Status};

    fn v5_header() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0x8000_0005u32.to_le_bytes());
        bytes.extend_from_slice(&0x26A7_270Au32.to_le_bytes());
        bytes.extend_from_slice(&0xc8e7_1055u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes
    }

    #[test]
    fn transparent_v5_output_is_leak() {
        let mut bytes = v5_header();
        bytes.push(0x00);
        bytes.push(0x01);
        bytes.extend_from_slice(&100_000i64.to_le_bytes());
        bytes.push(0x00);
        bytes.push(0x00);
        bytes.push(0x00);
        bytes.push(0x00);
        let verdict = inspect_bytes(&bytes, Policy::FullPrivacy).unwrap();
        assert_eq!(verdict.status, Status::Leak);
        assert_eq!(verdict.exit_code, 1);
        assert_eq!(verdict.codes, vec![Code::RevealedRecipient]);
        assert_eq!(verdict.transparent_output_zatoshis, vec![100_000]);
        assert!(verdict.transparent_input_zatoshis.is_none());
    }

    #[test]
    fn empty_v5_is_clean() {
        let mut bytes = v5_header();
        bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]);
        let verdict = inspect_bytes(&bytes, Policy::FullPrivacy).unwrap();
        assert_eq!(verdict.status, Status::Clean);
        assert!(verdict.codes.is_empty());
    }

    #[test]
    fn unknown_version_is_leak() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0x8000_0009u32.to_le_bytes());
        bytes.extend_from_slice(&1u32.to_le_bytes());
        let verdict = inspect_bytes(&bytes, Policy::NoPrivacy).unwrap();
        assert_eq!(verdict.status, Status::Leak);
        assert_eq!(verdict.codes, vec![Code::Unrecognized]);
    }

    #[test]
    fn truncated_bytes_are_a_usage_error() {
        let err = inspect_bytes(&[0x05], Policy::FullPrivacy).unwrap_err();
        assert!(err.contains("unreadable"));
    }
}
