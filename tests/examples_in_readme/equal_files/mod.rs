/// When compared files are equal, the exit code should be zero.
#[test]
fn _0001() {
  cli_assert::command!()
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}
