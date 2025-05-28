//! # biff
//!
//! Byte by byte file comparator.

mod cli;
mod comparator;

use crate::cli::{get_bytes, get_flag, get_matches, get_skip, get_str, get_value};
use crate::comparator::{compare, print_byte, ByteFormat, ComparisonOptions};
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
  let percent_limit: Option<f64> = get_value(&matches, "percent");
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

  let comparison_options = ComparisonOptions {
    file_name_1: file_name_1.clone(),
    skip_1,
    file_name_2: file_name_2.clone(),
    skip_2,
    max_bytes,
    marker_len: marker.len(),
    verbose,
    byte_format,
  };

  let Some(result) = compare(&comparison_options) else {
    return ExitCode::from(CODE_ERROR);
  };

  if marker != result.marker_1 {
    if !quiet {
      println!("marker not matched for file: {}", file_name_1);
    }
    return ExitCode::from(CODE_INVALID_MARKER);
  }

  if marker != result.marker_2 {
    if !quiet {
      println!("marker not matched for file: {}", file_name_2);
    }
    return ExitCode::from(CODE_INVALID_MARKER);
  }

  if let Some(limit) = percent_limit {
    let difference = result.counter as f64 * 100.0 / result.size as f64;
    return if difference > limit {
      if !quiet {
        println!(
          "{file_name_1} {file_name_2} differ: limit {}% exceeded by value {:.03}%",
          limit, difference
        );
      }
      ExitCode::from(CODE_DIFFERENT)
    } else {
      ExitCode::from(CODE_EQUAL)
    };
  }

  if let Some(limit) = absolute_limit {
    return if result.counter > limit {
      if !quiet {
        println!(
          "{file_name_1} {file_name_2} differ: limit {} exceeded by value {}",
          limit, result.counter
        );
      }
      ExitCode::from(CODE_DIFFERENT)
    } else {
      ExitCode::from(CODE_EQUAL)
    };
  }

  if result.counter > 0 {
    if !verbose && !quiet {
      print!(
        "{file_name_1} {file_name_2} differ: byte {}, line {}",
        result.first_difference_offset, result.first_difference_line
      );
      if print_bytes {
        print!(" is ");
        print_byte(result.first_difference_byte_1, byte_format);
        print!(" ");
        print_byte(result.first_difference_byte_2, byte_format);
      }
      println!();
    }
    return ExitCode::from(CODE_DIFFERENT);
  }
  ExitCode::from(CODE_EQUAL)
}
