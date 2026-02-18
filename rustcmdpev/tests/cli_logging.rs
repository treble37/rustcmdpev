use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run_with_args_and_stdin(args: &[&str], stdin_payload: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .args(args)
        .env_remove("RUST_LOG")
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

#[test]
fn verbose_mode_emits_info_logs() {
    let output = run_with_args_and_stdin(
        &["-v", "--format", "pretty", "--color", "never"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("INFO"));
    assert!(stdout.contains("starting rustcmdpev"));
}

#[test]
fn quiet_mode_suppresses_info_logs_on_success() {
    let output = run_with_args_and_stdin(
        &["-q", "--format", "pretty", "--color", "never"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("INFO"));
    assert!(!stdout.contains("starting rustcmdpev"));
}
