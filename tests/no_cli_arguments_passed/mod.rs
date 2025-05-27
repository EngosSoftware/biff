use super::*;
use assert_cmd::Command;

#[cfg(target_os = "linux")]
#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .assert()
    .code(2)
    .stdout("")
    .stderr(
      r#"error: the following required arguments were not provided:
  <FILE1>
  <FILE2>

Usage: biff <FILE1> <FILE2>

For more information, try '--help'.
"#,
    );
}

#[cfg(target_os = "windows")]
#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .assert()
    .code(2)
    .stdout("")
    .stderr(
      r#"error: the following required arguments were not provided:
  <FILE1>
  <FILE2>

Usage: biff.exe <FILE1> <FILE2>

For more information, try '--help'.
"#,
    );
}
