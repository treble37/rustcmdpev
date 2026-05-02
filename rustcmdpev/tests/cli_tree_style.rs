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

const PAYLOAD: &str = r#"[{"Plan":{"Node Type":"Hash Join","Total Cost":4.0,"Actual Total Time":1.0,"Actual Rows":2,"Actual Loops":1,"Plans":[{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":2,"Actual Loops":1}]},"Execution Time":2.0}]"#;

#[test]
fn ascii_tree_style_emits_only_ascii_box_drawings() {
    let output = run(&["--color", "never", "--tree-style", "ascii"], PAYLOAD);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hash Join"));
    assert!(!stdout.contains('│'));
    assert!(!stdout.contains('└'));
    assert!(!stdout.contains('├'));
}

#[test]
fn heavy_tree_style_uses_heavy_glyphs() {
    let output = run(&["--color", "never", "--tree-style", "heavy"], PAYLOAD);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains('┃'),
        "expected heavy vertical, got:\n{stdout}"
    );
}

#[test]
fn compat_rejects_non_unicode_tree_style() {
    let output = run(
        &["--color", "never", "--compat", "--tree-style", "ascii"],
        PAYLOAD,
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--compat"));
}
