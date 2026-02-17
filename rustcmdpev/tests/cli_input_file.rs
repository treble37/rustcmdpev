use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_file_path(suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_nanos();
    let pid = std::process::id();
    env::temp_dir().join(format!("rustcmdpev_{pid}_{nanos}_{suffix}.json"))
}

#[test]
fn input_file_valid_json_succeeds() {
    let path = unique_temp_file_path("valid");
    fs::write(&path, "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]")
        .expect("failed to write temp input file");

    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(&path)
        .output()
        .expect("failed to run rustcmdpev");

    let _ = fs::remove_file(&path);
    assert!(output.status.success());
}

#[test]
fn input_file_missing_returns_non_zero_and_actionable_error() {
    let path = unique_temp_file_path("missing");

    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(&path)
        .output()
        .expect("failed to run rustcmdpev");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("failed to read input file"));
}

#[test]
fn input_file_invalid_json_returns_non_zero_and_error() {
    let path = unique_temp_file_path("invalid");
    fs::write(&path, "not-json").expect("failed to write temp input file");

    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(&path)
        .output()
        .expect("failed to run rustcmdpev");

    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid JSON input"));
}
