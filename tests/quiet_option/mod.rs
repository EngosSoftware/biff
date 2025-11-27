#[test]
fn _0001() {
  cli_assert::command!()
    .arg("file1.txt")
    .arg("file2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .arg("-q")
    .arg("file1.txt")
    .arg("file2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0003() {
  cli_assert::command!()
    .arg("-s")
    .arg("file1.txt")
    .arg("file2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0004() {
  cli_assert::command!()
    .arg("-q")
    .arg("file3.txt")
    .arg("file1.txt")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("-s")
    .arg("file3.txt")
    .arg("file1.txt")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .arg("-q")
    .arg("-a")
    .arg("2")
    .arg("file4.txt")
    .arg("file5.txt")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!()
    .arg("-s")
    .arg("-a")
    .arg("2")
    .arg("file4.txt")
    .arg("file5.txt")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0008() {
  cli_assert::command!()
    .arg("-q")
    .arg("-a")
    .arg("50")
    .arg("file4.txt")
    .arg("file5.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0009() {
  cli_assert::command!()
    .arg("-q")
    .arg("-p")
    .arg("10")
    .arg("file4.txt")
    .arg("file5.txt")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0010() {
  cli_assert::command!()
    .arg("-s")
    .arg("-p")
    .arg("10")
    .arg("file4.txt")
    .arg("file5.txt")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0011() {
  cli_assert::command!()
    .arg("-q")
    .arg("-p")
    .arg("90")
    .arg("file4.txt")
    .arg("file5.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0012() {
  cli_assert::command!()
    .arg("-q")
    .arg("-m")
    .arg("414141")
    .arg("file1.txt")
    .arg("file6.txt")
    .code(3)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0013() {
  cli_assert::command!()
    .arg("-q")
    .arg("-m")
    .arg("414141")
    .arg("file6.txt")
    .arg("file1.txt")
    .code(3)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0014() {
  cli_assert::command!()
    .arg("-q")
    .arg("-m")
    .arg("414141")
    .arg("file6.txt")
    .arg("file6.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0015() {
  cli_assert::command!()
    .arg("-q")
    .arg("-p")
    .arg("0.7")
    .arg("-m")
    .arg("255044462d312e340a25")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}
