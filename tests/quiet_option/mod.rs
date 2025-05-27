use super::*;
use assert_cmd::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file1.txt")
    .arg("file2.txt")
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
    .arg("-q")
    .arg("file1.txt")
    .arg("file2.txt")
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
    .arg("-s")
    .arg("file1.txt")
    .arg("file2.txt")
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
    .arg("-q")
    .arg("file3.txt")
    .arg("file1.txt")
    .assert()
    .code(1)
    .stdout("")
    .stderr("");
}

#[test]
fn _0005() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-s")
    .arg("file3.txt")
    .arg("file1.txt")
    .assert()
    .code(1)
    .stdout("")
    .stderr("");
}

#[test]
fn _0006() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-a")
    .arg("2")
    .arg("file4.txt")
    .arg("file5.txt")
    .assert()
    .code(1)
    .stdout("")
    .stderr("");
}

#[test]
fn _0007() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-s")
    .arg("-a")
    .arg("2")
    .arg("file4.txt")
    .arg("file5.txt")
    .assert()
    .code(1)
    .stdout("")
    .stderr("");
}

#[test]
fn _0008() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-a")
    .arg("50")
    .arg("file4.txt")
    .arg("file5.txt")
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
    .arg("-q")
    .arg("-p")
    .arg("10")
    .arg("file4.txt")
    .arg("file5.txt")
    .assert()
    .code(1)
    .stdout("")
    .stderr("");
}

#[test]
fn _0010() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-s")
    .arg("-p")
    .arg("10")
    .arg("file4.txt")
    .arg("file5.txt")
    .assert()
    .code(1)
    .stdout("")
    .stderr("");
}

#[test]
fn _0011() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-p")
    .arg("90")
    .arg("file4.txt")
    .arg("file5.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0012() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-m")
    .arg("414141")
    .arg("file1.txt")
    .arg("file6.txt")
    .assert()
    .code(3)
    .stdout("")
    .stderr("");
}

#[test]
fn _0013() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-m")
    .arg("414141")
    .arg("file6.txt")
    .arg("file1.txt")
    .assert()
    .code(3)
    .stdout("")
    .stderr("");
}

#[test]
fn _0014() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-m")
    .arg("414141")
    .arg("file6.txt")
    .arg("file6.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

#[test]
fn _0015() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-p")
    .arg("0.7")
    .arg("-m")
    .arg("255044462d312e340a25")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(1)
    .stdout("")
    .stderr("");
}
