#[test]
fn _0001() {
  cli_assert::command!()
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
    .arg("-m")
    .arg("255044462d312e340a25")
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
    .arg("-q")
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
    .arg("-s")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("--quiet")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .arg("--silent")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!()
    .arg("-i")
    .arg("2")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0008() {
  cli_assert::command!()
    .arg("--ignore-initial")
    .arg("2")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0009() {
  cli_assert::command!()
    .arg("-i")
    .arg("6:5")
    .arg("file1.txt")
    .arg("file2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0010() {
  cli_assert::command!()
    .arg("-i")
    .arg("6:5")
    .arg("-n")
    .arg("6")
    .arg("file1.txt")
    .arg("file3.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0011() {
  cli_assert::command!()
    .arg("--ignore-initial")
    .arg("6:5")
    .arg("--bytes")
    .arg("6")
    .arg("file1.txt")
    .arg("file3.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}
