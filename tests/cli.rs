//! The binary a stranger runs. These checks lock the exit codes the README promises.

use std::process::Command;

fn leakline(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_leakline"))
        .args(args)
        .output()
        .expect("run leakline");
    let code = output.status.code().expect("exit code");
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    let stderr = String::from_utf8(output.stderr).expect("stderr");
    (code, stdout, stderr)
}

fn example(name: &str) -> String {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name)
        .to_str()
        .expect("path")
        .to_string()
}

#[test]
fn transparent_output_flips_with_policy() {
    let path = example("transparent-v5.hex");
    let (code, out, err) = leakline(&["--policy", "FullPrivacy", &path]);
    assert_eq!(code, 1, "{err}");
    assert!(out.contains("\"status\":\"LEAK\""));
    assert!(out.contains("REVEALED_RECIPIENT"));
    assert!(out.contains("\"transparent_input_zatoshis\":null"));

    let (code, out, _) = leakline(&["--policy", "AllowRevealedRecipients", &path]);
    assert_eq!(code, 0);
    assert!(out.contains("\"status\":\"CLEAN\""));
    assert!(out.contains("REVEALED_RECIPIENT"));
}

#[test]
fn crossing_flips_and_fee_stays_clean() {
    let crossing = example("crossing-v5.hex");
    let (code, out, _) = leakline(&["--policy", "FullPrivacy", &crossing]);
    assert_eq!(code, 1);
    assert!(out.contains("POOL_CROSSING"));
    let (code, out, _) = leakline(&["--policy", "AllowRevealedAmounts", &crossing]);
    assert_eq!(code, 0);
    assert!(out.contains("POOL_CROSSING"));

    let (code, out, _) = leakline(&["--policy", "FullPrivacy", &example("orchard-fee-v5.hex")]);
    assert_eq!(code, 0);
    assert!(out.contains("FEE_CANDIDATE"));
    assert!(out.contains("\"status\":\"CLEAN\""));
}

#[test]
fn unrecognized_stays_leak_under_no_privacy() {
    let (code, out, _) = leakline(&[
        "--policy",
        "NoPrivacy",
        &example("unknown-v9.hex"),
    ]);
    assert_eq!(code, 1);
    assert!(out.contains("UNRECOGNIZED"));
    assert!(out.contains("\"status\":\"LEAK\""));
}

#[test]
fn empty_transaction_and_pczt_are_clean() {
    for name in ["empty-v5.hex", "empty-nu6.pczt.hex"] {
        let (code, out, err) = leakline(&[&example(name)]);
        assert_eq!(code, 0, "{name} {err} {out}");
        assert!(out.contains("\"status\":\"CLEAN\""));
        assert!(out.contains("\"policy\":\"FullPrivacy\""));
    }
}

#[test]
fn usage_errors_exit_2() {
    let (code, _, err) = leakline(&["--policy", "NotAPolicy", &example("empty-v5.hex")]);
    assert_eq!(code, 2);
    assert!(err.contains("unknown policy"));

    let (code, _, _) = leakline(&["--help"]);
    assert_eq!(code, 0);

    let output = Command::new(env!("CARGO_BIN_EXE_leakline"))
        .arg(&example("truncated.hex"))
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(2));
}
