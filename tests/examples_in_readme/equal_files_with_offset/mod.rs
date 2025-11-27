/// When compared files are equal starting from specified offset,
/// the exit code should be zero.
#[test]
fn _0001() {
  cli_assert::command!()
    .arg("-i")
    .arg("2")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}
