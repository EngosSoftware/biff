//! # biff
//!
//! Byte by byte file comparator.

mod cli;
mod comparator;

use crate::cli::{get_bytes, get_flag, get_matches, get_skip, get_str, get_value};
use crate::comparator::{compare, print_byte, ByteFormat, ComparisonOptions, ComparisonResult};
use std::process::ExitCode;

const CODE_EQUAL: u8 = 0;
const CODE_DIFFERENT: u8 = 1;
const CODE_ERROR: u8 = 2;
const CODE_INVALID_MARKER: u8 = 3;

/// Main entrypoint of `biff` application.
fn main() -> ExitCode {
  let matches = get_matches();
  let file_name_1 = get_str(&matches, "FILE1").unwrap();
  let file_name_2 = get_str(&matches, "FILE2").unwrap();
  let marker = if let Some(marker_str) = get_str(&matches, "marker") {
    hex::decode(marker_str).unwrap()
  } else {
    vec![]
  };
  let percentage_limit: Option<f64> = get_value(&matches, "percent");
  let absolute_limit: Option<usize> = get_value(&matches, "absolute");
  let print_bytes = get_flag(&matches, "print-bytes");
  let byte_format = if get_flag(&matches, "octal") {
    ByteFormat::Octal
  } else if get_flag(&matches, "hexadecimal") {
    ByteFormat::Hex
  } else {
    ByteFormat::Decimal
  };
  let verbose = get_flag(&matches, "verbose");
  let quiet = get_flag(&matches, "quiet") || get_flag(&matches, "silent");
  let (skip_1, skip_2) = get_skip(&matches, "ignore-initial");
  let max_bytes = get_bytes(&matches, "bytes", usize::MAX);

  let options = ComparisonOptions {
    file_name_1,
    skip_1,
    file_name_2,
    skip_2,
    max_bytes,
    marker,
    verbose,
    quiet,
    byte_format,
    percentage_limit,
    absolute_limit,
    print_bytes,
  };

  match compare(&options) {
    ComparisonResult::Identical => ExitCode::from(CODE_EQUAL),
    ComparisonResult::SimilarPercentage(limit, difference) => {
      let (_, _) = (limit, difference);
      ExitCode::from(CODE_EQUAL)
    }
    ComparisonResult::SimilarAbsolute(limit, difference) => {
      let (_, _) = (limit, difference);
      ExitCode::from(CODE_EQUAL)
    }
    ComparisonResult::AbsoluteLimitExceeded(limit, difference) => {
      if !options.quiet {
        println!(
          "{} {} differ: limit {} exceeded by value {}",
          options.file_name_1, options.file_name_2, limit, difference
        );
      }
      ExitCode::from(CODE_DIFFERENT)
    }
    ComparisonResult::PercentageLimitExceeded(limit, difference) => {
      if !options.quiet {
        println!(
          "{} {} differ: limit {}% exceeded by value {:.03}%",
          options.file_name_1, options.file_name_2, limit, difference
        );
      }
      ExitCode::from(CODE_DIFFERENT)
    }
    ComparisonResult::Different(details) => {
      if !options.verbose && !options.quiet {
        print!(
          "{} {} differ: byte {}, line {}",
          options.file_name_1, options.file_name_2, details.first_difference_offset, details.first_difference_line
        );
        if options.print_bytes {
          print!(" is ");
          print_byte(details.first_difference_byte_1, options.byte_format);
          print!(" ");
          print_byte(details.first_difference_byte_2, options.byte_format);
        }
        println!();
      }
      ExitCode::from(CODE_DIFFERENT)
    }
    ComparisonResult::InvalidMarker(file_name, expected, actual) => {
      if !options.quiet {
        println!(
          "marker not matched for file: {}, expected: {}, actual: {}",
          file_name,
          hex::encode(expected),
          hex::encode(actual)
        );
      }
      ExitCode::from(CODE_INVALID_MARKER)
    }
    ComparisonResult::Error(reason) => {
      eprintln!("{}", reason);
      ExitCode::from(CODE_ERROR)
    }
  }
}
