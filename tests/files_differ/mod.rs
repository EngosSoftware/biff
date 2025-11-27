#[test]
fn _0001() {
  cli_assert::command!()
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(1)
    .stdout("file1.pdf file2.pdf differ: byte 97, line 6\n")
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
    .code(1)
    .stdout("file1.pdf file2.pdf differ: byte 97, line 6\n")
    .stderr("")
    .execute();
}

#[test]
fn _0003() {
  cli_assert::command!()
    .arg("file1.pdf")
    .arg("file2.txt")
    .code(1)
    .stdout("file1.pdf file2.txt differ: byte 1, line 1\n")
    .stderr("")
    .execute();
}

#[test]
fn _0004() {
  cli_assert::command!()
    .arg("-m")
    .arg("255044462d312e340a25")
    .arg("file1.pdf")
    .arg("file2.txt")
    .code(3)
    .stdout("marker not matched for file: file2.txt, expected: 255044462d312e340a25, actual: 4c6f72656d2069707375\n")
    .stderr("")
    .execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("-m")
    .arg("255044462d312e340a25")
    .arg("file2.txt")
    .arg("file1.pdf")
    .code(3)
    .stdout("marker not matched for file: file2.txt, expected: 255044462d312e340a25, actual: 4c6f72656d2069707375\n")
    .stderr("")
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .arg("file1.txt")
    .arg("file3.txt")
    .code(1)
    .stdout("file1.txt file3.txt differ: byte 22, line 1\n")
    .stderr("")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!()
    .arg("file3.txt")
    .arg("file1.txt")
    .code(1)
    .stdout("file3.txt file1.txt differ: byte 22, line 1\n")
    .stderr("")
    .execute();
}

#[test]
fn _0008() {
  cli_assert::command!()
    .arg("-p")
    .arg("0.01")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 0.01% exceeded by value 0.26%\n")
    .stderr("")
    .execute();
}

#[test]
fn _0009() {
  cli_assert::command!()
    .arg("--percent")
    .arg("0.01")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 0.01% exceeded by value 0.26%\n")
    .stderr("")
    .execute();
}

#[test]
fn _0010() {
  cli_assert::command!()
    .arg("-a")
    .arg("7")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 7 exceeded by value 57\n")
    .stderr("")
    .execute();
}

#[test]
fn _0011() {
  cli_assert::command!()
    .arg("--absolute")
    .arg("7")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 7 exceeded by value 57\n")
    .stderr("")
    .execute();
}

#[cfg(target_os = "linux")]
#[test]
fn _0012() {
  cli_assert::command!()
    .arg("file4.txt")
    .arg("file5.txt")
    .code(1)
    .stdout("file4.txt file5.txt differ: byte 22, line 3\n")
    .stderr("")
    .execute();
}

#[cfg(target_os = "windows")]
#[test]
fn _0013() {
  cli_assert::command!()
    .arg("file4.txt")
    .arg("file5.txt")
    .code(1)
    .stdout("file4.txt file5.txt differ: byte 23, line 2\n")
    .stderr("")
    .execute();
}

#[cfg(target_os = "linux")]
#[test]
fn _0014() {
  cli_assert::command!()
    .arg("file5.txt")
    .arg("file6.txt")
    .code(1)
    .stdout("file5.txt file6.txt differ: byte 22, line 2\n")
    .stderr("")
    .execute();
}

#[cfg(target_os = "windows")]
#[test]
fn _0015() {
  cli_assert::command!()
    .arg("file5.txt")
    .arg("file6.txt")
    .code(1)
    .stdout("file5.txt file6.txt differ: byte 23, line 2\n")
    .stderr("")
    .execute();
}

#[test]
fn _0016() {
  cli_assert::command!()
    .arg("-b")
    .arg("-d")
    .arg("file3.txt")
    .arg("file7.txt")
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 116 119\n")
    .stderr("")
    .execute();
}

#[test]
fn _0017() {
  cli_assert::command!()
    .arg("-b")
    .arg("--decimal")
    .arg("file3.txt")
    .arg("file7.txt")
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 116 119\n")
    .stderr("")
    .execute();
}

#[test]
fn _0018() {
  cli_assert::command!()
    .arg("-b")
    .arg("-o")
    .arg("file3.txt")
    .arg("file7.txt")
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 164 167\n")
    .stderr("")
    .execute();
}

#[test]
fn _0019() {
  cli_assert::command!()
    .arg("-b")
    .arg("--octal")
    .arg("file3.txt")
    .arg("file7.txt")
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 164 167\n")
    .stderr("")
    .execute();
}

#[test]
fn _0020() {
  cli_assert::command!()
    .arg("-b")
    .arg("-x")
    .arg("file3.txt")
    .arg("file7.txt")
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 74 77\n")
    .stderr("")
    .execute();
}

#[test]
fn _0021() {
  cli_assert::command!()
    .arg("-b")
    .arg("--hexadecimal")
    .arg("file3.txt")
    .arg("file7.txt")
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 74 77\n")
    .stderr("")
    .execute();
}

#[test]
fn _0022() {
  cli_assert::command!()
    .arg("--print-bytes")
    .arg("file8.txt")
    .arg("file9.txt")
    .code(1)
    .stdout("file8.txt file9.txt differ: byte 23, line 1 is 97 EOF\n")
    .stderr("")
    .execute();
}

#[test]
fn _0023() {
  cli_assert::command!()
    .arg("--print-bytes")
    .arg("file9.txt")
    .arg("file8.txt")
    .code(1)
    .stdout("file9.txt file8.txt differ: byte 23, line 1 is EOF 97\n")
    .stderr("")
    .execute();
}

#[test]
fn _0024() {
  cli_assert::command!()
    .arg("-l")
    .arg("file10.txt")
    .arg("file11.txt")
    .code(1)
    .stdout("1    97  65\n2    98  66\n3    99  67\n")
    .stderr("")
    .execute();
}

#[test]
fn _0025() {
  cli_assert::command!()
    .arg("-lo")
    .arg("file10.txt")
    .arg("file11.txt")
    .code(1)
    .stdout("1    141  101\n2    142  102\n3    143  103\n")
    .stderr("")
    .execute();
}

#[test]
fn _0026() {
  cli_assert::command!()
    .arg("-lx")
    .arg("file10.txt")
    .arg("file11.txt")
    .code(1)
    .stdout("1    61  41\n2    62  42\n3    63  43\n")
    .stderr("")
    .execute();
}

#[test]
fn _0027() {
  cli_assert::command!()
    .arg("-l")
    .arg("file10.txt")
    .arg("file12.txt")
    .code(1)
    .stdout("4    EOF  100\n5    EOF  101\n6    EOF  102\n7    EOF  103\n")
    .stderr("")
    .execute();
}

#[test]
fn _0028() {
  cli_assert::command!()
    .arg("-l")
    .arg("file12.txt")
    .arg("file10.txt")
    .code(1)
    .stdout("4    100  EOF\n5    101  EOF\n6    102  EOF\n7    103  EOF\n")
    .stderr("")
    .execute();
}
