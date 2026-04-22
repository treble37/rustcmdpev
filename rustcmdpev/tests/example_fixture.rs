use std::path::{Path, PathBuf};
use std::process::Command;

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
