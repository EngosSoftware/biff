use super::*;
use assert_cmd::Command;

#[test]
fn _0004() {
  let suffixes = ["kB", "k", "K", "KiB", "MB", "M", "MiB", "GB", "G", "GiB"];
  for suffix in suffixes {
    let offset = format!("1{}", suffix);
    let mut cmd = Command::cargo_bin("biff").unwrap();
    cmd
      .current_dir(current_dir(file!()))
      .arg("-i")
      .arg(offset)
      .arg("file1.txt")
      .arg("file2.txt")
      .assert()
      .code(0)
      .stdout("")
      .stderr("");
  }
}
