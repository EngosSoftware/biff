#[cfg(target_os = "linux")]
#[test]
fn _0001() {
  cli_assert::command!()
    .arg("a.txt")
    .arg("file.txt")
    .code(2)
    .stdout("")
    .stderr("Can not open file. Os { code: 2, kind: NotFound, message: \"No such file or directory\" }\n")
    .execute();
}

#[cfg(target_os = "windows")]
#[test]
fn _0002() {
  cli_assert::command!()
    .arg("a.txt")
    .arg("file.txt")
    .code(2)
    .stdout("")
    .stderr(
      "Can not open file. Os { code: 2, kind: NotFound, message: \"The system cannot find the file specified.\" }\n",
    )
    .execute();
}

#[cfg(target_os = "linux")]
#[test]
fn _0003() {
  cli_assert::command!()
    .arg("file.txt")
    .arg("a.txt")
    .code(2)
    .stderr("Can not open file. Os { code: 2, kind: NotFound, message: \"No such file or directory\" }\n")
    .execute();
}

#[cfg(target_os = "windows")]
#[test]
fn _0004() {
  cli_assert::command!()
    .arg("file.txt")
    .arg("a.txt")
    .code(2)
    .stderr(
      "Can not open file. Os { code: 2, kind: NotFound, message: \"The system cannot find the file specified.\" }\n",
    )
    .execute();
}

#[cfg(target_os = "linux")]
#[test]
fn _0005() {
  cli_assert::command!()
    .arg("/proc/self/mem")
    .arg("file.txt")
    .code(2)
    .stdout("")
    .stderr(
      "Reading bytes failed. Some(Err(Os { code: 5, kind: Uncategorized, message: \"Input/output error\" })) Some(Ok(76))\n",
    ).execute();
}

#[cfg(target_os = "linux")]
#[test]
fn _0006() {
  cli_assert::command!()
    .arg("file.txt")
    .arg("/proc/self/mem")
    .code(2)
    .stderr(
      "Reading bytes failed. Some(Ok(76)) Some(Err(Os { code: 5, kind: Uncategorized, message: \"Input/output error\" }))\n",
    )
    .execute();
}
