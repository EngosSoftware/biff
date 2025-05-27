use super::*;
use assert_cmd::Command;

/// Compared files differ in 2 bytes,
/// so the returned status code should be zero,
/// because the absolute threshold is 3.
#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-a")
    .arg("3")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

/// Compared files differ below percentage threshold,
/// so the returned status code should be zero.
#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-p")
    .arg("25.0")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}

/// Compared files differ below percentage threshold,
/// so the returned status code should be zero.
#[test]
fn _0003() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-q")
    .arg("-p")
    .arg("25.0")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .assert()
    .code(0)
    .stdout("")
    .stderr("");
}
