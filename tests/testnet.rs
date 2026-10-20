//! Real bytes from public Zcash testnet (NU6.3, v6, Ironwood). See examples/testnet/README.md.

use std::process::Command;

fn run(policy: &str, name: &str) -> (i32, String) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/testnet")
        .join(name);
    let output = Command::new(env!("CARGO_BIN_EXE_leakline"))
        .args(["--policy", policy])
        .arg(path)
        .output()
        .expect("run leakline");
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    (output.status.code().expect("exit code"), stdout)
}

#[test]
fn testnet_transparent_payment_leaks() {
    let (code, out) = run("FullPrivacy", "leak-transparent-v6.hex");
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("REVEALED_RECIPIENT"));
    assert!(out.contains("\"transparent_output_zatoshis\":[1000000]"));
    assert!(out.contains("\"ironwood_zatoshis\":1015000"));
    assert!(!out.contains("POOL_CROSSING"));

    let (code, _) = run("AllowRevealedRecipients", "leak-transparent-v6.hex");
    assert_eq!(code, 0);
}

#[test]
fn testnet_shielded_payment_is_clean() {
    let (code, out) = run("FullPrivacy", "clean-shielded-v6.hex");
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("\"codes\":[\"FEE_CANDIDATE\"]"));
    assert!(out.contains("\"transparent_output_zatoshis\":[]"));
    assert!(out.contains("\"ironwood_zatoshis\":10000"));
}

#[test]
fn testnet_pczt_is_caught_before_signing() {
    let (code, out) = run("FullPrivacy", "leak-transparent-v6.pczt.hex");
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("REVEALED_RECIPIENT"));
    assert!(out.contains("PCZT_METADATA"));
    assert!(out.contains("\"transparent_output_zatoshis\":[1000000]"));

    let (code, _) = run("AllowRevealedRecipients", "leak-transparent-v6.pczt.hex");
    assert_eq!(code, 0);
}

#[test]
fn testnet_shielded_pczt_is_clean_with_metadata_warning() {
    let (code, out) = run("FullPrivacy", "clean-shielded-v6.pczt.hex");
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("\"codes\":[\"FEE_CANDIDATE\",\"PCZT_METADATA\"]"));
    assert!(out.contains("\"transparent_output_zatoshis\":[]"));
}
