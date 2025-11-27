#[test]
fn _0001() {
  cli_assert::command!()
    .arg("-p")
    .arg("0.3")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .arg("--percent")
    .arg("0.3")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0003() {
  cli_assert::command!()
    .arg("-a")
    .arg("60")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0004() {
  cli_assert::command!()
    .arg("--absolute")
    .arg("60")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}
