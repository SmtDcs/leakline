//! Read a PCZT or a signed transaction with librustzcash. No hand-written bundle parser.

use zcash_primitives::transaction::{Transaction, TxVersion};
use zcash_protocol::{
    consensus::{BranchId, OrchardProtocolRevision},
    constants::{V5_TX_VERSION, V5_VERSION_GROUP_ID, V6_TX_VERSION, V6_VERSION_GROUP_ID},
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

    if !matches!(tx.version(), TxVersion::V4 | TxVersion::V5 | TxVersion::V6) {
        let mut exposure = Exposure::empty_shielded();
        exposure.recognized = false;
        return Ok(inspect(&exposure, policy));
    }

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
    if !pczt_supported(&pczt) {
        let mut exposure = Exposure::empty_shielded();
        exposure.recognized = false;
        return Ok(inspect(&exposure, policy));
    }
    let mut exposure = Exposure::empty_shielded();
    if !pczt.global().proprietary().is_empty() {
        exposure.pczt_metadata = true;
    }

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
        if !spend.proprietary().is_empty() {
            exposure.pczt_metadata = true;
        }
    }
    for output in pczt.sapling().outputs() {
        if output.user_address().is_some() || !output.proprietary().is_empty() {
            exposure.pczt_metadata = true;
        }
    }
    for bundle in [pczt.orchard(), pczt.ironwood()] {
        for action in bundle.actions() {
            let spend = action.spend();
            let output = action.output();
            if !spend.proprietary().is_empty()
                || spend.dummy_sk().is_some()
                || !output.proprietary().is_empty()
                || output.user_address().is_some()
            {
                exposure.pczt_metadata = true;
            }
        }
    }

    Ok(inspect(&exposure, policy))
}

fn pczt_supported(pczt: &pczt::Pczt) -> bool {
    let global = pczt.global();
    let Ok(branch) = BranchId::try_from(*global.consensus_branch_id()) else {
        return false;
    };
    let Some(revision) = branch.orchard_protocol_revision() else {
        return false;
    };
    let version_ok = match (*global.tx_version(), *global.version_group_id()) {
        (V5_TX_VERSION, V5_VERSION_GROUP_ID) => ironwood_empty(pczt.ironwood()),
        (V6_TX_VERSION, V6_VERSION_GROUP_ID) => revision >= OrchardProtocolRevision::V3,
        _ => false,
    };
    version_ok
}

fn ironwood_empty(bundle: &pczt::orchard::Bundle) -> bool {
    bundle.actions().is_empty() && *bundle.value_sum() == (0, false)
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

    #[test]
    fn v3_is_unrecognized() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0x8000_0003u32.to_le_bytes());
        bytes.extend_from_slice(&0x03C4_8270u32.to_le_bytes());
        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.push(0x00);
        let verdict = inspect_bytes(&bytes, Policy::NoPrivacy).unwrap();
        assert_eq!(verdict.status, Status::Leak);
        assert_eq!(verdict.codes, vec![Code::Unrecognized]);
    }

    #[test]
    fn empty_v6_is_clean() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0x8000_0006u32.to_le_bytes());
        bytes.extend_from_slice(&0xD884_B698u32.to_le_bytes());
        bytes.extend_from_slice(&0x37a5_165bu32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let verdict = inspect_bytes(&bytes, Policy::FullPrivacy).unwrap();
        assert_eq!(verdict.status, Status::Clean);
        assert!(verdict.codes.is_empty());
    }

    #[test]
    fn sapling_orchard_bytes_cross_under_two_policies() {
        let bytes = crossing_v5();
        let locked = inspect_bytes(&bytes, Policy::FullPrivacy).unwrap();
        assert_eq!(locked.status, Status::Leak);
        assert_eq!(locked.exit_code, 1);
        assert_eq!(locked.codes, vec![Code::PoolCrossing]);
        assert_eq!(locked.sapling_zatoshis, 5_0000_0000);
        assert_eq!(locked.orchard_zatoshis, -4_9900_0000);
        assert!(locked.transparent_output_zatoshis.is_empty());
        let opened = inspect_bytes(&bytes, Policy::AllowRevealedAmounts).unwrap();
        assert_eq!(opened.status, Status::Clean);
        assert_eq!(opened.exit_code, 0);
        assert_eq!(opened.codes, vec![Code::PoolCrossing]);
    }

    #[test]
    fn empty_nu6_pczt_is_clean() {
        let pczt = pczt::roles::creator::Creator::new(
            u32::from(BranchId::Nu6),
            0,
            133,
            Some([0; 32]),
            Some([0; 32]),
        )
        .expect("creator")
        .build()
        .expect("build")
        .serialize()
        .expect("serialize");
        let verdict = inspect_bytes(&pczt, Policy::FullPrivacy).unwrap();
        assert_eq!(verdict.status, Status::Clean);
    }

    fn crossing_v5() -> Vec<u8> {
        use group::{Group, GroupEncoding};
        let jubjub = jubjub::SubgroupPoint::generator().to_bytes();
        let pallas = pasta_curves::pallas::Point::generator().to_bytes();
        redjubjub::VerificationKey::<redjubjub::SpendAuth>::try_from(jubjub)
            .expect("jubjub generator is a verification key");

        let mut bytes = v5_header();
        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.push(0x01);
        bytes.extend_from_slice(&jubjub);
        bytes.extend_from_slice(&[1; 32]);
        bytes.extend_from_slice(&jubjub);
        bytes.push(0x00);
        bytes.extend_from_slice(&5_0000_0000i64.to_le_bytes());
        bytes.extend_from_slice(&[0; 32]);
        bytes.extend_from_slice(&[0; 192]);
        bytes.extend_from_slice(&[0; 64]);
        bytes.extend_from_slice(&[0; 64]);
        bytes.push(0x01);
        bytes.extend_from_slice(&pallas);
        bytes.extend_from_slice(&pallas);
        bytes.extend_from_slice(&pallas);
        bytes.extend_from_slice(&[0; 32]);
        bytes.extend_from_slice(&pallas);
        bytes.extend_from_slice(&[0; 580]);
        bytes.extend_from_slice(&[0; 80]);
        bytes.push(0x03);
        bytes.extend_from_slice(&(-4_9900_0000i64).to_le_bytes());
        bytes.extend_from_slice(&[0; 32]);
        bytes.push(0x00);
        bytes.extend_from_slice(&[0; 64]);
        bytes.extend_from_slice(&[0; 64]);
        bytes
    }
}
