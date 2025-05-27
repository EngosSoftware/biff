use super::*;
use assert_cmd::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
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
    .arg("-m")
    .arg("255044462d312e340a25")
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
    .arg("-q")
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
    .arg("-s")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0005() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--quiet")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0006() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--silent")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0007() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-i")
    .arg("2")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0008() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--ignore-initial")
    .arg("2")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0009() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-i")
    .arg("6:5")
    .arg("file1.txt")
    .arg("file2.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0010() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-i")
    .arg("6:5")
    .arg("-n")
    .arg("6")
    .arg("file1.txt")
    .arg("file3.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0011() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--ignore-initial")
    .arg("6:5")
    .arg("--bytes")
    .arg("6")
    .arg("file1.txt")
    .arg("file3.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}
