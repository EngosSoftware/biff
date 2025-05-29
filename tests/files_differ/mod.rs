use super::*;
use assert_cmd::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(1)
    .stdout("file1.pdf file2.pdf differ: byte 97, line 6\n")
    .stderr("");
}

#[test]
fn _0002() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-m")
    .arg("255044462d312e340a25")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(1)
    .stdout("file1.pdf file2.pdf differ: byte 97, line 6\n")
    .stderr("");
}

#[test]
fn _0003() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file1.pdf")
    .arg("file2.txt")
    .assert()
    .code(1)
    .stdout("file1.pdf file2.txt differ: byte 1, line 1\n")
    .stderr("");
}

#[test]
fn _0004() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-m")
    .arg("255044462d312e340a25")
    .arg("file1.pdf")
    .arg("file2.txt")
    .assert()
    .code(3)
    .stdout("marker not matched for file: file2.txt, expected: 255044462d312e340a25, actual: 4c6f72656d2069707375\n")
    .stderr("");
}

#[test]
fn _0005() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-m")
    .arg("255044462d312e340a25")
    .arg("file2.txt")
    .arg("file1.pdf")
    .assert()
    .code(3)
    .stdout("marker not matched for file: file2.txt, expected: 255044462d312e340a25, actual: 4c6f72656d2069707375\n")
    .stderr("");
}

#[test]
fn _0006() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file1.txt")
    .arg("file3.txt")
    .assert()
    .code(1)
    .stdout("file1.txt file3.txt differ: byte 22, line 1\n")
    .stderr("");
}

#[test]
fn _0007() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file3.txt")
    .arg("file1.txt")
    .assert()
    .code(1)
    .stdout("file3.txt file1.txt differ: byte 22, line 1\n")
    .stderr("");
}

#[test]
fn _0008() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-p")
    .arg("0.01")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 0.01% exceeded by value 0.26%\n")
    .stderr("");
}

#[test]
fn _0009() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--percent")
    .arg("0.01")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 0.01% exceeded by value 0.26%\n")
    .stderr("");
}

#[test]
fn _0010() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-a")
    .arg("7")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 7 exceeded by value 57\n")
    .stderr("");
}

#[test]
fn _0011() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--absolute")
    .arg("7")
    .arg("file1.pdf")
    .arg("file2.pdf")
    .assert()
    .code(1)
    .stdout("file1.pdf file2.pdf differ: limit 7 exceeded by value 57\n")
    .stderr("");
}

#[cfg(target_os = "linux")]
#[test]
fn _0012() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file4.txt")
    .arg("file5.txt")
    .assert()
    .code(1)
    .stdout("file4.txt file5.txt differ: byte 22, line 3\n")
    .stderr("");
}

#[cfg(target_os = "windows")]
#[test]
fn _0013() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file4.txt")
    .arg("file5.txt")
    .assert()
    .code(1)
    .stdout("file4.txt file5.txt differ: byte 23, line 2\n")
    .stderr("");
}

#[cfg(target_os = "linux")]
#[test]
fn _0014() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file5.txt")
    .arg("file6.txt")
    .assert()
    .code(1)
    .stdout("file5.txt file6.txt differ: byte 22, line 2\n")
    .stderr("");
}

#[cfg(target_os = "windows")]
#[test]
fn _0015() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("file5.txt")
    .arg("file6.txt")
    .assert()
    .code(1)
    .stdout("file5.txt file6.txt differ: byte 23, line 2\n")
    .stderr("");
}

#[test]
fn _0016() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("-d")
    .arg("file3.txt")
    .arg("file7.txt")
    .assert()
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 116 119\n")
    .stderr("");
}

#[test]
fn _0017() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("--decimal")
    .arg("file3.txt")
    .arg("file7.txt")
    .assert()
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 116 119\n")
    .stderr("");
}

#[test]
fn _0018() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("-o")
    .arg("file3.txt")
    .arg("file7.txt")
    .assert()
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 164 167\n")
    .stderr("");
}

#[test]
fn _0019() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("--octal")
    .arg("file3.txt")
    .arg("file7.txt")
    .assert()
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 164 167\n")
    .stderr("");
}

#[test]
fn _0020() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("-x")
    .arg("file3.txt")
    .arg("file7.txt")
    .assert()
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 74 77\n")
    .stderr("");
}

#[test]
fn _0021() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("--hexadecimal")
    .arg("file3.txt")
    .arg("file7.txt")
    .assert()
    .code(1)
    .stdout("file3.txt file7.txt differ: byte 21, line 1 is 74 77\n")
    .stderr("");
}

#[test]
fn _0022() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--print-bytes")
    .arg("file8.txt")
    .arg("file9.txt")
    .assert()
    .code(1)
    .stdout("file8.txt file9.txt differ: byte 23, line 1 is 97 EOF\n")
    .stderr("");
}

#[test]
fn _0023() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("--print-bytes")
    .arg("file9.txt")
    .arg("file8.txt")
    .assert()
    .code(1)
    .stdout("file9.txt file8.txt differ: byte 23, line 1 is EOF 97\n")
    .stderr("");
}

#[test]
fn _0024() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-l")
    .arg("file10.txt")
    .arg("file11.txt")
    .assert()
    .code(1)
    .stdout("1    97  65\n2    98  66\n3    99  67\n")
    .stderr("");
}

#[test]
fn _0025() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-lo")
    .arg("file10.txt")
    .arg("file11.txt")
    .assert()
    .code(1)
    .stdout("1    141  101\n2    142  102\n3    143  103\n")
    .stderr("");
}

#[test]
fn _0026() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-lx")
    .arg("file10.txt")
    .arg("file11.txt")
    .assert()
    .code(1)
    .stdout("1    61  41\n2    62  42\n3    63  43\n")
    .stderr("");
}

#[test]
fn _0027() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-l")
    .arg("file10.txt")
    .arg("file12.txt")
    .assert()
    .code(1)
    .stdout("4    EOF  100\n5    EOF  101\n6    EOF  102\n7    EOF  103\n")
    .stderr("");
}

#[test]
fn _0028() {
  let mut cmd = Command::cargo_bin("biff").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-l")
    .arg("file12.txt")
    .arg("file10.txt")
    .assert()
    .code(1)
    .stdout("4    100  EOF\n5    101  EOF\n6    102  EOF\n7    103  EOF\n")
    .stderr("");
}
