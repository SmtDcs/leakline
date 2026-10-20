//! Leakline: say whether a Zcash transaction or PCZT would reveal an amount, a sender,
//! or a recipient, before it is signed or broadcast. Offline; no keys.
//!
//! ```
//! use leakline::{inspect_hex, Code, Policy, Status};
//!
//! let hex = include_str!("../examples/testnet/leak-transparent-v6.pczt.hex");
//! let verdict = inspect_hex(hex, Policy::FullPrivacy).unwrap();
//! assert_eq!(verdict.status, Status::Leak);
//! assert!(verdict.codes.contains(&Code::RevealedRecipient));
//! ```

mod parse;
mod policy;

pub use parse::{inspect_bytes, inspect_hex};
pub use policy::{inspect, Code, Exposure, Policy, Status, Verdict};
