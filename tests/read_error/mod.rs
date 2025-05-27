use super::*;
use assert_cmd::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("/proc/self/mem")
    .arg("file.txt")
    .assert()
    .code(2)
    .stdout("")
    .stderr(
      "Unexpected: Some(Err(Os { code: 5, kind: Uncategorized, message: \"Input/output error\" })), Some(Ok(76))\n",
    );
}

#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file.txt")
    .arg("/proc/self/mem")
    .assert()
    .code(2)
    .stderr(
      "Unexpected: Some(Ok(76)), Some(Err(Os { code: 5, kind: Uncategorized, message: \"Input/output error\" }))\n",
    );
}
