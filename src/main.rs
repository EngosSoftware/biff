//! # biff
//!
//! Byte by byte file comparator.

mod cli;
mod comparator;

use crate::cli::{get_bytes, get_flag, get_matches, get_skip, get_str, get_value};
use crate::comparator::{compare, ComparisonOptions, ComparisonResult};
use std::process::ExitCode;

const CODE_EQUAL: u8 = 0;
const CODE_DIFFERENT: u8 = 1;
const CODE_ERROR: u8 = 2;
const CODE_INVALID_MARKER: u8 = 3;

/// Format of the byte value when printed.
#[derive(Debug, Default, Copy, Clone)]
pub(crate) enum ByteFormat {
  /// Decimal value.
  Decimal,
  /// Octal value.
  Octal,
  /// Hexadecimal value.
  #[default]
  Hex,
}

/// Displays byte value in specified format.
pub(crate) fn print_byte(value: Option<u8>, byte_format: ByteFormat) {
  if let Some(b) = value {
    match byte_format {
      ByteFormat::Decimal => print!("{}", b),
      ByteFormat::Octal => print!("{:o}", b),
      ByteFormat::Hex => print!("{:x}", b),
    }
  } else {
    print!("EOF")
  }
}

/// Displays byte differences.
pub(crate) fn print_diff(offset: usize, b1: Option<u8>, b2: Option<u8>, byte_format: ByteFormat) {
  print!("{:<5}", offset);
  print_byte(b1, byte_format);
  print!("  ");
  print_byte(b2, byte_format);
  println!()
}

/// Main entrypoint of `biff` application.
fn main() -> ExitCode {
  let matches = get_matches();
  let file_name_1 = get_str(&matches, "FILE1").unwrap();
  let file_name_2 = get_str(&matches, "FILE2").unwrap();
  let marker = if let Some(marker_str) = get_str(&matches, "marker") {
    match hex::decode(marker_str) {
      Ok(marker_bytes) => marker_bytes,
      Err(reason) => {
        eprintln!("Invalid marker. {}", reason);
        return ExitCode::from(CODE_ERROR);
      }
    }
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
    percentage_limit,
    absolute_limit,
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
      if !quiet {
        println!(
          "{} {} differ: limit {} exceeded by value {}",
          options.file_name_1, options.file_name_2, limit, difference
        );
      }
      ExitCode::from(CODE_DIFFERENT)
    }
    ComparisonResult::PercentageLimitExceeded(limit, difference) => {
      if !quiet {
        println!(
          "{} {} differ: limit {}% exceeded by value {:.03}%",
          options.file_name_1, options.file_name_2, limit, difference
        );
      }
      ExitCode::from(CODE_DIFFERENT)
    }
    ComparisonResult::Different(details) => {
      if !verbose && !quiet {
        print!(
          "{} {} differ: byte {}, line {}",
          options.file_name_1, options.file_name_2, details.first_difference_offset, details.first_difference_line
        );
        if print_bytes {
          print!(" is ");
          print_byte(details.first_difference_byte_1, byte_format);
          print!(" ");
          print_byte(details.first_difference_byte_2, byte_format);
        }
        println!();
      }
      if verbose {
        for (offset, byte_1, byte_2) in details.differences {
          print_diff(offset, byte_1, byte_2, byte_format);
        }
      }
      ExitCode::from(CODE_DIFFERENT)
    }
    ComparisonResult::InvalidMarker(file_name, expected, actual) => {
      if !quiet {
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
