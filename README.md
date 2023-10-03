# biff

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][coc-badge]](https://github.com/wisbery/biff/blob/main/CODE_OF_CONDUCT.md)

[crates-badge]: https://img.shields.io/crates/v/biff.svg
[crates-url]: https://crates.io/crates/biff
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/wisbery/biff/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://github.com/wisbery/biff/blob/main/LICENSE-APACHE
[build-badge-linux]: https://github.com/wisbery/biff/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/wisbery/biff/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/wisbery/biff/actions/workflows/build-macos.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg

## Overview

Compare two files byte by byte.

**biff** compares two files byte by byte, and if they differ, tells the first byte and line number where they differ.
Additionally, when option `-l` or `--verbose` is set, **biff** will display all differing bytes.

**biff** may also skip some initial bytes in compared files or compare only up to requested number of bytes.

When comparing binary files, **biff** may check the beginning bytes if they match the specified _marker_, so only
files having such marker will be compared.

There are also some popular cases, when compared files differ only in few bytes, but generally may be considered
as _similar_ and this is quite alright. For such comparisons **biff** may use the absolute total number of acceptable
differences (option `-a` or `--absolute`) or percentage difference limit (option `-p` or `--percent`).

The most popular use case of such _similarity comparisons_ are PDF files generated 
from the same HTML files using Headless Chrome. 

The best fit for **biff** are test cases with output values being files.  

## Installation

```
$ cargo install biff
```

## Usage

Display short description of **biff**'s options:

```
$ biff -h
```

Display detailed description of **biff**'s options:

```
$ biff --help
```

## Examples

<!--- see: bbt/tests/examples_in_readme/EXAMPLE_1 -->
### Files are equal

```shell
$ cat sample1.txt
Lorem ipsum.
$ cat sample2.txt
Lorem ipsum.

$ biff sample1.txt sample2.txt
$ echo $?
0
```

<!--- see: bbt/tests/examples_in_readme/EXAMPLE_2 -->
### Files are equal but starting from the 3rd byte

```shell
$ cat sample1.txt
LOrem ipsum.
$ cat sample2.txt
MArem ipsum.

$ biff -i 2 sample1.txt sample2.txt
$ echo $?
0
```

<!--- see: bbt/tests/examples_in_readme/EXAMPLE_3 -->
### Files are similar (difference limits)

```shell
$ cat sample1.txt
LoreM ipsum.
$ cat sample2.txt
LoRem ipsuM.

$ biff -a 3 sample1.txt sample2.txt
$ echo $?
0

$ biff -p 26.8 sample1.txt sample2.txt
$ echo $?
0

$ biff -q -p 26.8 sample1.txt sample2.txt
$ echo $?
0
```

<!--- see: bbt/tests/examples_in_readme/EXAMPLE_4 -->
### Files differ

```shell
$ cat sample1.txt
LoreM ipsum.
$ cat sample2.txt
LoRem ipsuM.

$ biff sample1.txt sample2.txt
sample1.txt sample2.txt differ: byte 3, line 1
$ echo $?
1

$ biff -b sample1.txt sample2.txt
sample1.txt sample2.txt differ: byte 3, line 1 is 114 82
$ echo $?
1

$ biff -bx sample1.txt sample2.txt
sample1.txt sample2.txt differ: byte 3, line 1 is 72 52
$ echo $?
1

$ biff -q sample1.txt sample2.txt
$ echo $?
1

$ biff -l sample1.txt sample2.txt
3    114  82
5    77  109
11   109  77
$ echo $?
1

$ biff -p 0.1 sample1.txt sample2.txt
sample1.txt sample2.txt differ: limit 0.1% exceeded by value 25.000%
$ echo $?
1

$ biff -a 1 sample1.txt sample2.txt
sample1.txt sample2.txt differ: limit 1 exceeded by value 3
$ echo $?
1
```

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/wisbery/biff/blob/main/LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/wisbery/biff/blob/main/LICENSE-APACHE))

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
