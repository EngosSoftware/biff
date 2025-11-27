#[test]
fn _0001() {
  cli_assert::command!()
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(1)
    .stdout("sample1.txt sample2.txt differ: byte 3, line 1\n")
    .stderr("")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .arg("-b")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(1)
    .stdout("sample1.txt sample2.txt differ: byte 3, line 1 is 114 82\n")
    .stderr("")
    .execute();
}

#[test]
fn _0003() {
  cli_assert::command!()
    .arg("-b")
    .arg("-x")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(1)
    .stdout("sample1.txt sample2.txt differ: byte 3, line 1 is 72 52\n")
    .stderr("")
    .execute();
}

#[test]
fn _0004() {
  cli_assert::command!()
    .arg("-q")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(1)
    .stdout("")
    .stderr("")
    .execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("-l")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(1)
    .stdout("3    114  82\n5    77  109\n11   109  77\n")
    .stderr("")
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .arg("-p")
    .arg("0.1")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(1)
    .stdout("sample1.txt sample2.txt differ: limit 0.1% exceeded by value 25.00%\n")
    .stderr("")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!()
    .arg("-a")
    .arg("1")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(1)
    .stdout("sample1.txt sample2.txt differ: limit 1 exceeded by value 3\n")
    .stderr("")
    .execute();
}
