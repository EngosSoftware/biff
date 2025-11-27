/// Compared files differ in 2 bytes,
/// so the returned status code should be zero,
/// because the absolute threshold is 3.
#[test]
fn _0001() {
  cli_assert::command!()
    .arg("-a")
    .arg("3")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

/// Compared files differ below percentage threshold,
/// so the returned status code should be zero.
#[test]
fn _0002() {
  cli_assert::command!()
    .arg("-p")
    .arg("25.0")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}

/// Compared files differ below percentage threshold,
/// so the returned status code should be zero.
#[test]
fn _0003() {
  cli_assert::command!()
    .arg("-q")
    .arg("-p")
    .arg("25.0")
    .arg("sample1.txt")
    .arg("sample2.txt")
    .code(0)
    .stdout("")
    .stderr("")
    .execute();
}
