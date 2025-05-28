//! # biff
//!
//! Byte by byte file comparator.

use crate::cli::{get_bytes, get_flag, get_matches, get_skip, get_str, get_value};
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::ExitCode;

mod cli;

/// Line feed definition.
const LF: u8 = 10;

pub const CODE_EQUAL: u8 = 0;
pub const CODE_DIFFERENT: u8 = 1;
pub const CODE_ERROR: u8 = 2;
pub const CODE_INVALID_MARKER: u8 = 3;

/// Result details of file comparison.
struct ComparisonResult {
  /// Total number of different bytes in compared files.
  counter: usize,
  /// Size of the files, when files differ in sizes, than the bigger file size is returned.
  size: usize,
  /// Header marker of the first file.
  marker_1: Vec<u8>,
  /// Header marker of the second file.
  marker_2: Vec<u8>,
  /// Byte number of the first difference.
  first_difference_offset: usize,
  /// Line number of the first difference.
  first_difference_line: usize,
  /// First differing byte from first file.
  first_difference_byte_1: Option<u8>,
  /// First differing byte from second file.
  first_difference_byte_2: Option<u8>,
}

impl ComparisonResult {
  /// Creates new comparison result.
  fn new() -> Self {
    Self {
      counter: 0,
      size: 0,
      marker_1: vec![],
      marker_2: vec![],
      first_difference_offset: 0,
      first_difference_line: 1,
      first_difference_byte_1: None,
      first_difference_byte_2: None,
    }
  }
}

/// Format of the byte value when printed.
enum ByteFormat {
  /// Decimal value.
  Decimal,
  /// Octal value.
  Octal,
  /// Hexadecimal value.
  Hex,
}

/// Displays byte value in specified format.
fn print_byte(value: Option<u8>, byte_format: &ByteFormat) {
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
fn print_diff(offset: usize, b1: Option<u8>, b2: Option<u8>, verbose: bool, byte_format: &ByteFormat) {
  if verbose {
    print!("{:<5}", offset);
    print_byte(b1, byte_format);
    print!("  ");
    print_byte(b2, byte_format);
    println!()
  }
}

/// Compares two files.
#[allow(clippy::too_many_arguments)]
fn compare(
  file_name_1: &str,
  skip_1: usize,
  file_name_2: &str,
  skip_2: usize,
  max_bytes: usize,
  marker_len: usize,
  verbose: bool,
  byte_format: &ByteFormat,
) -> Option<ComparisonResult> {
  let mut result = ComparisonResult::new();
  let file_1 = match File::open(file_name_1) {
    Ok(file) => file,
    Err(reason) => {
      eprintln!("Unexpected: {:?}", reason);
      return None;
    }
  };
  let file_2 = match File::open(file_name_2) {
    Ok(file) => file,
    Err(reason) => {
      eprintln!("Unexpected: {:?}", reason);
      return None;
    }
  };
  let buf_1 = BufReader::new(file_1);
  let buf_2 = BufReader::new(file_2);
  let mut iter_1 = buf_1.bytes().skip(skip_1).take(max_bytes);
  let mut iter_2 = buf_2.bytes().skip(skip_2).take(max_bytes);
  let mut first_difference = false;
  loop {
    match (iter_1.next(), iter_2.next()) {
      (Some(Ok(b1)), Some(Ok(b2))) => {
        if !first_difference && b1 == LF {
          result.first_difference_line += 1;
        }
        if result.size < marker_len {
          result.marker_1.push(b1);
          result.marker_2.push(b2);
        }
        result.size += 1;
        if b1 != b2 {
          if !first_difference {
            result.first_difference_offset = result.size;
            result.first_difference_byte_1 = Some(b1);
            result.first_difference_byte_2 = Some(b2);
          }
          print_diff(result.size, Some(b1), Some(b2), verbose, byte_format);
          result.counter += 1;
          first_difference = true;
        }
      }
      (None, Some(Ok(b2))) => {
        result.size += 1;
        if !first_difference {
          result.first_difference_offset = result.size;
          result.first_difference_byte_2 = Some(b2);
        }
        print_diff(result.size, None, Some(b2), verbose, byte_format);
        result.counter += 1;
        first_difference = true;
      }
      (Some(Ok(b1)), None) => {
        if !first_difference && b1 == LF {
          result.first_difference_line += 1;
        }
        result.size += 1;
        if !first_difference {
          result.first_difference_offset = result.size;
          result.first_difference_byte_1 = Some(b1);
        }
        print_diff(result.size, Some(b1), None, verbose, byte_format);
        result.counter += 1;
        first_difference = true;
      }
      (None, None) => break,
      (v1, v2) => {
        eprintln!("Unexpected: {:?}, {:?}", v1, v2);
        return None;
      }
    }
  }
  Some(result)
}

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

  let Some(result) = compare(
    &file_name_1,
    skip_1,
    &file_name_2,
    skip_2,
    max_bytes,
    marker.len(),
    verbose,
    &byte_format,
  ) else {
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
        print_byte(result.first_difference_byte_1, &byte_format);
        print!(" ");
        print_byte(result.first_difference_byte_2, &byte_format);
      }
      println!();
    }
    return ExitCode::from(CODE_DIFFERENT);
  }
  ExitCode::from(CODE_EQUAL)
}
