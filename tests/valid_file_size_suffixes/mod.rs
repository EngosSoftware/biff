#[test]
fn _0004() {
  let suffixes = ["kB", "k", "K", "KiB", "MB", "M", "MiB", "GB", "G", "GiB"];
  for suffix in suffixes {
    let offset = format!("1{}", suffix);
    cli_assert::command!()
      .arg("-i")
      .arg(offset)
      .arg("file1.txt")
      .arg("file2.txt")
      .code(0)
      .stdout("")
      .stderr("")
      .execute();
  }
}
