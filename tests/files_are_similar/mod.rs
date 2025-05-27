use super::*;
use assert_cmd::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-p")
    .arg("0.3")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--percent")
    .arg("0.3")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0003() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-a")
    .arg("60")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0004() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--absolute")
    .arg("60")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}
