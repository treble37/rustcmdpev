use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{fs, io::Write};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn bundled_example_json_fixture_succeeds() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(repo_root().join("example.json"))
        .output()
        .expect("failed to run rustcmdpev");

    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid utf-8");
    assert!(stdout.contains("Seq Scan"));
}

#[test]
fn bundled_example_json_fixture_succeeds_via_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run rustcmdpev");

    child
        .stdin
        .take()
        .expect("stdin should be available")
        .write_all(&fs::read(repo_root().join("example.json")).expect("read bundled fixture"))
        .expect("write bundled fixture to stdin");

    let output = child.wait_with_output().expect("collect rustcmdpev output");
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid utf-8");
    assert!(stdout.contains("Seq Scan"));
}
