use super::*;
use assert_cmd::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-p")
    .arg("percentage_must_be_number")
    .arg("file1.txt")
    .arg("file2.txt")
    .assert()
    .code(1)
    .stdout("file1.txt file2.txt differ: byte 12, line 1\n")
    .stderr("");
}

#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-i")
    .arg(":5")
    .arg("file1.txt")
    .arg("file2.txt")
    .assert()
    .code(1)
    .stdout("file1.txt file2.txt differ: byte 1, line 1\n")
    .stderr("");
}

#[test]
fn _0003() {
  let suffixes = ["kB", "k", "K", "KiB", "MB", "M", "MiB", "GB", "G", "GiB"];
  for suffix in suffixes {
    let offset = format!("{}{}", usize::MAX, suffix);
    let mut cmd = Command::cargo_bin("biff").unwrap();
    cmd
      .current_dir(current_dir(file!()))
      .arg("-i")
      .arg(offset)
      .arg("file1.txt")
      .arg("file2.txt")
      .assert()
      .code(1)
      .stdout("file1.txt file2.txt differ: byte 12, line 1\n")
      .stderr("");
  }
}

#[test]
fn _0004() {
  let offsets = ["not-a-number", "a2kB"];
  for offset in offsets {
    let mut cmd = Command::cargo_bin("biff").unwrap();
    cmd
      .current_dir(current_dir(file!()))
      .arg("-i")
      .arg(offset)
      .arg("file1.txt")
      .arg("file2.txt")
      .assert()
      .code(1)
      .stdout("file1.txt file2.txt differ: byte 12, line 1\n")
      .stderr("");
  }
}

#[test]
fn _0005() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-m")
    .arg("01xy")
    .arg("file1.txt")
    .arg("file2.txt")
    .assert()
    .code(2)
    .stdout("")
    .stderr("Invalid marker. Invalid character \'x\' at position 2\n");
}
