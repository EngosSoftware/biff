use super::*;
use assert_cmd::Command;

/// When compared files are equal, the exit code should be zero.
#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}
