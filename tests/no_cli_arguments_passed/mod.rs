#[cfg(not(target_os = "windows"))]
#[test]
fn _0001() {
  cli_assert::command!()
    .code(2)
    .stdout("")
    .stderr(
      r#"error: the following required arguments were not provided:
  <FILE1>
  <FILE2>

Usage: biff <FILE1> <FILE2>

For more information, try '--help'.
"#,
    )
    .execute();
}

#[cfg(target_os = "windows")]
#[test]
fn _0002() {
  cli_assert::command!()
    .code(2)
    .stdout("")
    .stderr(
      r#"error: the following required arguments were not provided:
  <FILE1>
  <FILE2>

Usage: biff.exe <FILE1> <FILE2>

For more information, try '--help'.
"#,
    )
    .execute();
}
