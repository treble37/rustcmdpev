use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str], stdin_payload: &str) -> Output {
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

// JSON has no PostgreSQL Version metadata and uses the legacy
// "IO Read Time" / "IO Write Time" labels (PG <13).
const LEGACY_IO_PAYLOAD: &str = r#"[{"Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":1,"Actual Loops":1,"IO Read Time":2.5,"IO Write Time":0.75}}]"#;

#[test]
fn postgres_version_hint_propagates_into_json_output() {
    let output = run(
        &[
            "--color",
            "never",
            "--format",
            "json",
            "--postgres-version",
            "12",
        ],
        LEGACY_IO_PAYLOAD,
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"postgres_version\": \"12\""),
        "expected postgres_version field in json output, got:\n{stdout}"
    );
    assert!(
        stdout.contains("\"io_read_time\": 2.5"),
        "expected legacy io_read_time to surface, got:\n{stdout}"
    );
}

#[test]
fn no_postgres_version_hint_leaves_field_omitted() {
    let output = run(&["--color", "never", "--format", "json"], LEGACY_IO_PAYLOAD);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\"postgres_version\""),
        "did not expect postgres_version field, got:\n{stdout}"
    );
}

#[test]
fn invalid_postgres_version_hint_does_not_fail() {
    let output = run(
        &[
            "--color",
            "never",
            "--format",
            "json",
            "--postgres-version",
            "garbage",
        ],
        LEGACY_IO_PAYLOAD,
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
