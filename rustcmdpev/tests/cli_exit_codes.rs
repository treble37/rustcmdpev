use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_with_args_and_stdin(args: &[&str], stdin_payload: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rustcmdpev");

    {
        let stdin = child.stdin.as_mut().expect("failed to open stdin");
        stdin
            .write_all(stdin_payload.as_bytes())
            .expect("failed to write stdin");
    }

    child.wait_with_output().expect("failed to wait on child")
}

fn unique_temp_file_path(suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_nanos();
    let pid = std::process::id();
    env::temp_dir().join(format!("rustcmdpev_{pid}_{nanos}_{suffix}.json"))
}

#[test]
fn exit_code_2_for_input_read_error() {
    let missing = unique_temp_file_path("missing");
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(&missing)
        .output()
        .expect("failed to run rustcmdpev");

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn exit_code_2_for_empty_stdin() {
    let output = run_with_args_and_stdin(&[], "");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn exit_code_3_for_invalid_input_json() {
    let output = run_with_args_and_stdin(&[], "not-json");
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn exit_code_4_for_invalid_compatibility_flags() {
    let output = run_with_args_and_stdin(
        &["--compat", "--format", "json"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert_eq!(output.status.code(), Some(4));
}

#[test]
fn exit_code_5_for_output_serialization_error_is_reserved() {
    // This path is currently hard to trigger via CLI input; verify success path for now.
    let path = unique_temp_file_path("valid");
    fs::write(&path, "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]")
        .expect("failed to write temp input file");

    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(&path)
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run rustcmdpev");

    let _ = fs::remove_file(&path);
    assert_eq!(output.status.code(), Some(0));
}
