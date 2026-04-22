use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("parity")
        .join(name)
}

#[test]
fn parity_fixture_succeeds_with_exit_code_zero() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--compat")
        .arg("--input")
        .arg(fixture_path("example.json"))
        .output()
        .expect("failed to run rustcmdpev");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn invalid_parity_json_fixture_returns_exit_code_three() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--compat")
        .arg("--input")
        .arg(fixture_path("invalid_json.json"))
        .output()
        .expect("failed to run rustcmdpev");

    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn invalid_parity_shape_fixture_returns_exit_code_three() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--compat")
        .arg("--input")
        .arg(fixture_path("invalid_shape.json"))
        .output()
        .expect("failed to run rustcmdpev");

    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn missing_parity_fixture_returns_exit_code_two() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--compat")
        .arg("--input")
        .arg(fixture_path("missing.json"))
        .output()
        .expect("failed to run rustcmdpev");

    assert_eq!(output.status.code(), Some(2));
}
