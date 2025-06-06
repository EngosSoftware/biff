//! # Command-line arguments

use clap::{arg, command, ArgAction, ArgGroup, ArgMatches};
use std::str::FromStr;

const HELP_ABSOLUTE: &str = "Acceptable absolute difference limit";
const LONG_HELP_ABSOLUTE: &str = r#"Acceptable absolute difference limit.
Specify acceptable maximum difference limit between compared files, expressed
as an <ABSOLUTE> number. When the number of differences between two files
is less or equal to <ABSOLUTE> value, files are considered as equal and
status code 0 is returned. When the number of differences exceeds this limit,
then files are considered as not equal and status code 1 is returned."#;

const HELP_PRINT_BYTES: &str = "Display differing bytes";
const LONG_HELP_PRINT_BYTES: &str = r#"Display the values of differing bytes. The first number is the differing byte
from the first file, the second number is the differing value from the second file.
By default, the displayed byte values are decimal numbers. This can be adjusted
using options -o --octal or -x --hexadecimal."#;

const HELP_DECIMAL: &str = "Display byte values as decimal numbers";
const LONG_HELP_DECIMAL: &str = r#"Display byte values as decimal numbers. This option forces
all displayed byte values to be converted to decimal numbers."#;

const HELP_IGNORE_INITIAL: &str = "Ignore any differences in the initial <SKIP> bytes of the input files";
const LONG_HELP_IGNORE_INITIAL: &str = r#"Ignore any differences in the initial <SKIP> bytes of the input files.
When <SKIP> is a single number, the same number of initial bytes are skipped
in both files. When <SKIP> has a format SKIP1:SKIP2, where SKIP1 and SKIP2
are both numbers, then in the first file are SKIP1 initial bytes skipped
and in the second file are SKIP2 bytes omitted. The number of bytes to be
skipped may have a suffix that defines the number multiplicator.
Recognized suffixes are: kB, k, K, KiB, MB, M, MiB, GB, G, GiB."#;

const HELP_VERBOSE: &str = "Display byte numbers and values of all differing bytes";
const LONG_HELP_VERBOSE: &str = r#"Display byte numbers and values of all differing bytes. All differing bytes
are displayed line by line, specifying the byte number and differing values.
First value is taken from the first compared file, the second value is taken
from the second file. By default, the byte values are displayed as decimal
numbers. This can be adjusted using options -o --octal or -x --hexadecimal.
When one of the compared files is shorter than the other, then instead
the byte value the text EOF is displayed."#;

const HELP_MARKER: &str = "Check file header marker before comparing files";
const LONG_HELP_MARKER: &str = r#"Check the file header marker before starting the comparison.
When both compared files do not begin with specified <MARKER>,
then the comparison is not performed and the status code 3 is returned.
<MARKER> may have any length and should be in the form of hexadecimal text
defining the consecutive marker's bytes."#;

const HELP_BYTES: &str = "Compare at most <COUNT> input bytes";
const LONG_HELP_BYTES: &str = r#"Compare at most <COUNT> input bytes.
<COUNT> is a number with optional suffix that defines the multiplicator.
Recognized suffixes are: kB, k, K, KiB, MB, M, MiB, GB, G, GiB."#;

const HELP_OCTAL: &str = "Display byte values as octal numbers";
const LONG_HELP_OCTAL: &str = r#"Display byte values as octal numbers. This option forces
all displayed byte values to be converted to octal numbers."#;

const HELP_PERCENT: &str = "Acceptable percent difference limit";
const LONG_HELP_PERCENT: &str = r#"Acceptable percent difference limit.
Specify acceptable maximum difference limit between compared files,
expressed as <PERCENT> value. When the number of differences between two files
is less or equal to <PERCENT> value, files are considered as equal and
status code 0 is returned. When the number of differences exceeds this limit,
then files are considered as not equal and status code 1 is returned."#;

const HELP_QUIET: &str = "Suppress all normal output";
const LONG_HELP_QUIET: &str = r#"Suppress all normal output. No messages will be printed to standard output.
The result of the comparison may be checked using the returned status code.
Returned status codes are:
  0 - files are equal,
  1 - files differ,
  2 - an error occurred,
  3 - invalid file marker."#;

const HELP_HEXADECIMAL: &str = "Display byte values as hexadecimal numbers";
const LONG_HELP_HEXADECIMAL: &str = r#"Display byte values as hexadecimal numbers. This option forces
all displayed byte values to be converted to hexadecimal numbers."#;

const HELP_SILENT: &str = "Same as --quiet";
const LONG_HELP_SILENT: &str = "Same as --quiet option.";

const HELP_FILE_1: &str = "First file to be compared with the second file";
const HELP_FILE_2: &str = "Second file to be compared with the first file";

/// Returns command-line matches.
pub fn get_matches() -> ArgMatches {
  command!()
    .arg(
      arg!(-a --absolute <ABSOLUTE>)
        .help(HELP_ABSOLUTE)
        .long_help(LONG_HELP_ABSOLUTE.to_string())
        .action(ArgAction::Set),
    )
    .arg(
      arg!(--"print-bytes")
        .short('b')
        .help(HELP_PRINT_BYTES)
        .long_help(LONG_HELP_PRINT_BYTES)
        .action(ArgAction::SetTrue),
    )
    .arg(
      arg!(--decimal)
        .short('d')
        .help(HELP_DECIMAL)
        .long_help(LONG_HELP_DECIMAL)
        .action(ArgAction::SetTrue),
    )
    .arg(
      arg!(-i --"ignore-initial" <SKIP>)
        .short('i')
        .help(HELP_IGNORE_INITIAL)
        .long_help(LONG_HELP_IGNORE_INITIAL)
        .action(ArgAction::Set),
    )
    .arg(
      arg!(--verbose)
        .short('l')
        .help(HELP_VERBOSE)
        .long_help(LONG_HELP_VERBOSE)
        .action(ArgAction::SetTrue),
    )
    .arg(
      arg!(-m --marker <MARKER>)
        .help(HELP_MARKER)
        .long_help(LONG_HELP_MARKER)
        .action(ArgAction::Set),
    )
    .arg(
      arg!(-i --"bytes" <COUNT>)
        .short('n')
        .help(HELP_BYTES)
        .long_help(LONG_HELP_BYTES)
        .action(ArgAction::Set),
    )
    .arg(
      arg!(--octal)
        .short('o')
        .help(HELP_OCTAL)
        .long_help(LONG_HELP_OCTAL)
        .action(ArgAction::SetTrue),
    )
    .arg(
      arg!(-p --percent <PERCENT>)
        .help(HELP_PERCENT)
        .long_help(LONG_HELP_PERCENT)
        .action(ArgAction::Set),
    )
    .arg(
      arg!(--quiet)
        .short('q')
        .help(HELP_QUIET)
        .long_help(LONG_HELP_QUIET)
        .action(ArgAction::SetTrue),
    )
    .arg(
      arg!(--silent)
        .short('s')
        .help(HELP_SILENT)
        .long_help(LONG_HELP_SILENT)
        .action(ArgAction::SetTrue),
    )
    .arg(
      arg!(--hexadecimal)
        .short('x')
        .help(HELP_HEXADECIMAL)
        .long_help(LONG_HELP_HEXADECIMAL)
        .action(ArgAction::SetTrue),
    )
    .group(ArgGroup::new("limits").arg("percent").arg("absolute"))
    .group(ArgGroup::new("format").arg("decimal").arg("octal").arg("hexadecimal"))
    .group(ArgGroup::new("verbosity").arg("verbose").arg("quiet").arg("silent"))
    .arg(arg!(<FILE1>).help(HELP_FILE_1).required(true).index(1))
    .arg(arg!(<FILE2>).help(HELP_FILE_2).required(true).index(2))
    .get_matches()
}

/// Returns matched value from command-line arguments.
pub fn get_value<T: FromStr>(matches: &ArgMatches, id: &str) -> Option<T> {
  if let Some(s) = matches.get_one::<String>(id).cloned() {
    if let Ok(value) = s.parse::<T>() {
      return Some(value);
    }
  }
  None
}

/// Returns matched string from command-line arguments.
pub fn get_str(matches: &ArgMatches, id: &str) -> Option<String> {
  matches.get_one::<String>(id).cloned()
}

/// Returns matched flag from command-line arguments.
pub fn get_flag(matches: &ArgMatches, id: &str) -> bool {
  matches.get_flag(id)
}

/// Returns matched skip bytes from command-line arguments.
pub fn get_skip(matches: &ArgMatches, id: &str) -> (Option<usize>, Option<usize>) {
  if let Some(s) = matches.get_one::<String>(id) {
    return if s.contains(':') {
      let mut parts = s.split(':');
      let skip_1 = parse_bytes(parts.next().unwrap());
      let skip_2 = parse_bytes(parts.next().unwrap());
      (skip_1, skip_2)
    } else {
      let skip = parse_bytes(s);
      (skip, skip)
    };
  }
  (None, None)
}

/// Returns matched bytes from command-line arguments.
pub fn get_bytes(matches: &ArgMatches, id: &str) -> Option<usize> {
  matches.get_one::<String>(id).and_then(|s| parse_bytes(s))
}

/// Parses a number with suffix, returns default value when error occurs.
fn parse_bytes(s: &str) -> Option<usize> {
  if let Some(prefix) = s.strip_suffix("kB") {
    if let Some(value) = multiplied(prefix, 1_000) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix('k') {
    if let Some(value) = multiplied(prefix, 1_024) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix('K') {
    if let Some(value) = multiplied(prefix, 1_024) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix("KiB") {
    if let Some(value) = multiplied(prefix, 1_024) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix("MB") {
    if let Some(value) = multiplied(prefix, 1_000_000) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix('M') {
    if let Some(value) = multiplied(prefix, 1_048_576) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix("MiB") {
    if let Some(value) = multiplied(prefix, 1_048_576) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix("GB") {
    if let Some(value) = multiplied(prefix, 1_000_000_000) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix('G') {
    if let Some(value) = multiplied(prefix, 1_073_741_824) {
      return Some(value);
    }
  }
  if let Some(prefix) = s.strip_suffix("GiB") {
    if let Some(value) = multiplied(prefix, 1_073_741_824) {
      return Some(value);
    }
  }
  if let Ok(value) = s.trim().parse::<usize>() {
    return Some(value);
  }
  None
}

fn multiplied(s: &str, multiplier: usize) -> Option<usize> {
  if let Ok(n) = s.trim().parse::<usize>() {
    if let Some(value) = n.checked_mul(multiplier) {
      return Some(value);
    }
  }
  None
}
