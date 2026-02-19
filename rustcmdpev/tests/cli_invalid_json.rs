use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_with_stdin(stdin_payload: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
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
fn invalid_json_stdin_returns_code_3_and_actionable_error() {
    let output = run_with_stdin("not-json");
    assert_eq!(output.status.code(), Some(3));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid JSON input"));
    assert!(stderr.contains("Ensure input is a PostgreSQL EXPLAIN FORMAT JSON array"));
}

#[test]
fn invalid_json_input_file_returns_code_3_and_actionable_error() {
    let path = unique_temp_file_path("invalid_json");
    fs::write(&path, "not-json").expect("failed to write temp input file");

    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(&path)
        .output()
        .expect("failed to run rustcmdpev");

    let _ = fs::remove_file(&path);
    assert_eq!(output.status.code(), Some(3));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid JSON input"));
    assert!(stderr.contains("Ensure input is a PostgreSQL EXPLAIN FORMAT JSON array"));
}
