//! Decision table from `hackathon/10-pair-system.md`.
//!
//! This module does not parse bytes and does not guess transparent change.

/// zcashd `privacyPolicy` names. A looser policy allows what the stricter one allows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Policy {
    FullPrivacy,
    AllowRevealedAmounts,
    AllowRevealedRecipients,
    AllowRevealedSenders,
    AllowFullyTransparent,
    AllowLinkingAccountAddresses,
    NoPrivacy,
}

impl Policy {
    pub fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "FullPrivacy" => Self::FullPrivacy,
            "AllowRevealedAmounts" => Self::AllowRevealedAmounts,
            "AllowRevealedRecipients" => Self::AllowRevealedRecipients,
            "AllowRevealedSenders" => Self::AllowRevealedSenders,
            "AllowFullyTransparent" => Self::AllowFullyTransparent,
            "AllowLinkingAccountAddresses" => Self::AllowLinkingAccountAddresses,
            "NoPrivacy" => Self::NoPrivacy,
            _ => return None,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FullPrivacy => "FullPrivacy",
            Self::AllowRevealedAmounts => "AllowRevealedAmounts",
            Self::AllowRevealedRecipients => "AllowRevealedRecipients",
            Self::AllowRevealedSenders => "AllowRevealedSenders",
            Self::AllowFullyTransparent => "AllowFullyTransparent",
            Self::AllowLinkingAccountAddresses => "AllowLinkingAccountAddresses",
            Self::NoPrivacy => "NoPrivacy",
        }
    }

    fn allows(self, code: Code) -> bool {
        if matches!(code, Code::FeeCandidate | Code::PcztMetadata) {
            return true;
        }
        if code == Code::Unrecognized {
            return false;
        }
        match self {
            Self::FullPrivacy => false,
            Self::NoPrivacy => true,
            Self::AllowRevealedAmounts => code == Code::PoolCrossing,
            Self::AllowRevealedRecipients => {
                matches!(code, Code::PoolCrossing | Code::RevealedRecipient)
            }
            Self::AllowRevealedSenders | Self::AllowLinkingAccountAddresses => {
                matches!(code, Code::PoolCrossing | Code::RevealedSender)
            }
            Self::AllowFullyTransparent => matches!(
                code,
                Code::PoolCrossing | Code::RevealedRecipient | Code::RevealedSender
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    RevealedSender,
    RevealedRecipient,
    PoolCrossing,
    FeeCandidate,
    PcztMetadata,
    Unrecognized,
}

impl Code {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RevealedSender => "REVEALED_SENDER",
            Self::RevealedRecipient => "REVEALED_RECIPIENT",
            Self::PoolCrossing => "POOL_CROSSING",
            Self::FeeCandidate => "FEE_CANDIDATE",
            Self::PcztMetadata => "PCZT_METADATA",
            Self::Unrecognized => "UNRECOGNIZED",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Clean,
    Leak,
}

/// Fields the parser is allowed to fill. Amounts stay `None` when the bytes do not carry them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exposure {
    /// `false` when the version or a bundle is not one this build can read.
    pub recognized: bool,
    pub transparent_input_count: u32,
    /// Set only when every transparent input value is present, which is a PCZT case.
    pub transparent_input_zatoshis: Option<i64>,
    pub transparent_output_zatoshis: Vec<i64>,
    /// Indexes PCZT marked as change. They stay public recipients under FullPrivacy.
    pub change_output_indexes: Vec<u32>,
    pub sprout_zatoshis: i64,
    pub sapling_zatoshis: i64,
    pub orchard_zatoshis: i64,
    pub ironwood_zatoshis: i64,
    pub pczt_metadata: bool,
}

impl Exposure {
    pub fn empty_shielded() -> Self {
        Self {
            recognized: true,
            transparent_input_count: 0,
            transparent_input_zatoshis: None,
            transparent_output_zatoshis: Vec::new(),
            change_output_indexes: Vec::new(),
            sprout_zatoshis: 0,
            sapling_zatoshis: 0,
            orchard_zatoshis: 0,
            ironwood_zatoshis: 0,
            pczt_metadata: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verdict {
    pub status: Status,
    pub policy: Policy,
    pub codes: Vec<Code>,
    pub exit_code: i32,
    pub transparent_output_zatoshis: Vec<i64>,
    pub transparent_input_zatoshis: Option<i64>,
    pub sprout_zatoshis: i64,
    pub sapling_zatoshis: i64,
    pub orchard_zatoshis: i64,
    pub ironwood_zatoshis: i64,
}

impl Verdict {
    pub fn to_json(&self) -> String {
        let codes = self
            .codes
            .iter()
            .map(|c| format!("\"{}\"", c.as_str()))
            .collect::<Vec<_>>()
            .join(",");
        let outputs = self
            .transparent_output_zatoshis
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let inputs = match self.transparent_input_zatoshis {
            Some(n) => n.to_string(),
            None => "null".to_string(),
        };
        let status = match self.status {
            Status::Clean => "CLEAN",
            Status::Leak => "LEAK",
        };
        format!(
            "{{\"status\":\"{status}\",\"policy\":\"{policy}\",\"exit\":{exit},\"codes\":[{codes}],\"transparent_input_zatoshis\":{inputs},\"transparent_output_zatoshis\":[{outputs}],\"sprout_zatoshis\":{sprout},\"sapling_zatoshis\":{sapling},\"orchard_zatoshis\":{orchard},\"ironwood_zatoshis\":{ironwood}}}",
            policy = self.policy.as_str(),
            exit = self.exit_code,
            sprout = self.sprout_zatoshis,
            sapling = self.sapling_zatoshis,
            orchard = self.orchard_zatoshis,
            ironwood = self.ironwood_zatoshis,
        )
    }
}

pub fn inspect(exposure: &Exposure, policy: Policy) -> Verdict {
    let mut codes = Vec::new();
    if !exposure.recognized {
        codes.push(Code::Unrecognized);
    } else {
        if exposure.transparent_input_count > 0 {
            codes.push(Code::RevealedSender);
        }
        if !exposure.transparent_output_zatoshis.is_empty() {
            codes.push(Code::RevealedRecipient);
        }
        let pools = [
            exposure.sprout_zatoshis,
            exposure.sapling_zatoshis,
            exposure.orchard_zatoshis,
            exposure.ironwood_zatoshis,
        ];
        let nonzero = pools.iter().filter(|v| **v != 0).count();
        let transparent = exposure.transparent_input_count > 0
            || !exposure.transparent_output_zatoshis.is_empty();
        if nonzero >= 2 {
            codes.push(Code::PoolCrossing);
        } else if nonzero == 1 && !transparent {
            codes.push(Code::FeeCandidate);
        }
        if exposure.pczt_metadata {
            codes.push(Code::PcztMetadata);
        }
    }

    let leak = codes.iter().any(|code| !policy.allows(*code));
    let status = if leak { Status::Leak } else { Status::Clean };
    Verdict {
        status,
        policy,
        exit_code: if leak { 1 } else { 0 },
        codes,
        transparent_output_zatoshis: exposure.transparent_output_zatoshis.clone(),
        transparent_input_zatoshis: if exposure.recognized {
            exposure.transparent_input_zatoshis
        } else {
            None
        },
        sprout_zatoshis: exposure.sprout_zatoshis,
        sapling_zatoshis: exposure.sapling_zatoshis,
        orchard_zatoshis: exposure.orchard_zatoshis,
        ironwood_zatoshis: exposure.ironwood_zatoshis,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crossing() -> Exposure {
        let mut tx = Exposure::empty_shielded();
        tx.sapling_zatoshis = 5_0000_0000;
        tx.orchard_zatoshis = -4_9900_0000;
        tx
    }

    #[test]
    fn transparent_output_leaks_under_full_privacy() {
        let mut tx = Exposure::empty_shielded();
        tx.transparent_output_zatoshis = vec![100_000];
        let verdict = inspect(&tx, Policy::FullPrivacy);
        assert_eq!(verdict.status, Status::Leak);
        assert_eq!(verdict.exit_code, 1);
        assert_eq!(verdict.codes, vec![Code::RevealedRecipient]);
        assert!(verdict.transparent_input_zatoshis.is_none());
    }

    #[test]
    fn pool_crossing_without_transparent_parts_is_leak_then_clean() {
        let tx = crossing();
        let locked = inspect(&tx, Policy::FullPrivacy);
        assert_eq!(locked.status, Status::Leak);
        assert_eq!(locked.codes, vec![Code::PoolCrossing]);
        assert_eq!(locked.sapling_zatoshis, 5_0000_0000);
        let opened = inspect(&tx, Policy::AllowRevealedAmounts);
        assert_eq!(opened.status, Status::Clean);
        assert_eq!(opened.exit_code, 0);
        assert_eq!(opened.codes, vec![Code::PoolCrossing]);
    }

    #[test]
    fn orchard_only_is_clean() {
        let mut tx = Exposure::empty_shielded();
        tx.orchard_zatoshis = 10_000;
        let verdict = inspect(&tx, Policy::FullPrivacy);
        assert_eq!(verdict.status, Status::Clean);
        assert_eq!(verdict.codes, vec![Code::FeeCandidate]);

        tx.orchard_zatoshis = 0;
        let still = inspect(&tx, Policy::FullPrivacy);
        assert_eq!(still.status, Status::Clean);
        assert!(still.codes.is_empty());
    }

    #[test]
    fn fee_sized_second_pool_is_still_crossing() {
        let mut tx = Exposure::empty_shielded();
        tx.orchard_zatoshis = 5_0000_0000;
        tx.ironwood_zatoshis = -4_9990_0000;
        let verdict = inspect(&tx, Policy::FullPrivacy);
        assert_eq!(verdict.codes, vec![Code::PoolCrossing]);
        assert_eq!(verdict.status, Status::Leak);
    }

    #[test]
    fn sender_and_recipient_follow_different_policies() {
        let mut tx = Exposure::empty_shielded();
        tx.transparent_input_count = 1;
        tx.transparent_output_zatoshis = vec![50_000];
        assert!(inspect(&tx, Policy::AllowRevealedRecipients)
            .codes
            .contains(&Code::RevealedSender));
        assert_eq!(
            inspect(&tx, Policy::AllowRevealedRecipients).status,
            Status::Leak
        );
        tx.transparent_output_zatoshis.clear();
        assert_eq!(
            inspect(&tx, Policy::AllowRevealedSenders).status,
            Status::Clean
        );
        assert_eq!(inspect(&tx, Policy::FullPrivacy).status, Status::Leak);
        assert_eq!(
            inspect(&tx, Policy::AllowLinkingAccountAddresses).status,
            Status::Clean
        );
        tx.transparent_output_zatoshis = vec![50_000];
        assert_eq!(
            inspect(&tx, Policy::AllowLinkingAccountAddresses).status,
            Status::Leak
        );
        assert!(inspect(&tx, Policy::FullPrivacy)
            .transparent_input_zatoshis
            .is_none());
    }

    #[test]
    fn unrecognized_is_leak_and_hides_invented_amounts() {
        let mut tx = crossing();
        tx.recognized = false;
        tx.transparent_input_count = 1;
        tx.transparent_input_zatoshis = Some(9);
        let verdict = inspect(&tx, Policy::NoPrivacy);
        assert_eq!(verdict.status, Status::Leak);
        assert_eq!(verdict.codes, vec![Code::Unrecognized]);
        assert!(verdict.transparent_input_zatoshis.is_none());
    }

    #[test]
    fn metadata_warning_does_not_flip_clean() {
        let mut tx = Exposure::empty_shielded();
        tx.pczt_metadata = true;
        let verdict = inspect(&tx, Policy::FullPrivacy);
        assert_eq!(verdict.status, Status::Clean);
        assert_eq!(verdict.codes, vec![Code::PcztMetadata]);
    }
}
