use super::*;
use assert_cmd::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(1)
    .stdout("sample1.txt sample2.txt differ: byte 3, line 1\n")
    .stderr("");
}

#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(1)
    .stdout("sample1.txt sample2.txt differ: byte 3, line 1 is 114 82\n")
    .stderr("");
}

#[test]
fn _0003() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("-x")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(1)
    .stdout("sample1.txt sample2.txt differ: byte 3, line 1 is 72 52\n")
    .stderr("");
}

#[test]
fn _0004() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("sample1.txt")
    .arg("sample2.txt")
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
    .arg("-l")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(1)
    .stdout("3    114  82\n5    77  109\n11   109  77\n")
    .stderr("");
}

#[test]
fn _0006() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-p")
    .arg("0.1")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(1)
    .stdout("sample1.txt sample2.txt differ: limit 0.1% exceeded by value 25.000%\n")
    .stderr("");
}

#[test]
fn _0007() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-a")
    .arg("1")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(1)
    .stdout("sample1.txt sample2.txt differ: limit 1 exceeded by value 3\n")
    .stderr("");
}
