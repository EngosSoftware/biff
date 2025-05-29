use super::*;
use assert_cmd::Command;

#[cfg(target_os = "linux")]
#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("a.txt")
    .arg("file.txt")
    .assert()
    .code(2)
    .stdout("")
    .stderr("Can not open file. Os { code: 2, kind: NotFound, message: \"No such file or directory\" }\n");
}

#[cfg(target_os = "windows")]
#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("a.txt")
    .arg("file.txt")
    .assert()
    .code(2)
    .stdout("")
    .stderr(
      "Can not open file. Os { code: 2, kind: NotFound, message: \"The system cannot find the file specified.\" }\n",
    );
}

#[cfg(target_os = "linux")]
#[test]
fn _0003() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file.txt")
    .arg("a.txt")
    .assert()
    .code(2)
    .stderr("Can not open file. Os { code: 2, kind: NotFound, message: \"No such file or directory\" }\n");
}

#[cfg(target_os = "windows")]
#[test]
fn _0004() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file.txt")
    .arg("a.txt")
    .assert()
    .code(2)
    .stderr(
      "Can not open file. Os { code: 2, kind: NotFound, message: \"The system cannot find the file specified.\" }\n",
    );
}

#[cfg(target_os = "linux")]
#[test]
fn _0005() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("/proc/self/mem")
    .arg("file.txt")
    .assert()
    .code(2)
    .stdout("")
    .stderr(
      "Reading bytes failed. Some(Err(Os { code: 5, kind: Uncategorized, message: \"Input/output error\" })) Some(Ok(76))\n",
    );
}

#[cfg(target_os = "linux")]
#[test]
fn _0006() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file.txt")
    .arg("/proc/self/mem")
    .assert()
    .code(2)
    .stderr(
      "Reading bytes failed. Some(Ok(76)) Some(Err(Os { code: 5, kind: Uncategorized, message: \"Input/output error\" }))\n",
    );
}
