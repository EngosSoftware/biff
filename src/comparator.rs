use std::fs::File;
use std::io::{BufReader, Read};

const LF: u8 = b'\n';

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
pub(crate) fn print_diff(offset: usize, b1: Option<u8>, b2: Option<u8>, verbose: bool, byte_format: ByteFormat) {
  if verbose {
    print!("{:<5}", offset);
    print_byte(b1, byte_format);
    print!("  ");
    print_byte(b2, byte_format);
    println!()
  }
}

#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct ComparisonOptions {
  pub file_name_1: String,
  pub skip_1: usize,
  pub file_name_2: String,
  pub skip_2: usize,
  pub max_bytes: usize,
  pub marker_len: usize,
  pub verbose: bool,
  pub byte_format: ByteFormat,
}

/// Result details of file comparison.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ComparisonResult {
  /// Total number of different bytes in compared files.
  pub counter: usize,
  /// Size of the files, when files differ in sizes, the bigger file size is returned.
  pub size: usize,
  /// Header marker of the first file.
  pub marker_1: Vec<u8>,
  /// Header marker of the second file.
  pub marker_2: Vec<u8>,
  /// Byte number of the first difference.
  pub first_difference_offset: usize,
  /// Line number of the first difference.
  pub first_difference_line: usize,
  /// First differing byte from the first file.
  pub first_difference_byte_1: Option<u8>,
  /// First differing byte from the second file.
  pub first_difference_byte_2: Option<u8>,
}

impl Default for ComparisonResult {
  fn default() -> Self {
    Self::new()
  }
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

/// Compares two files.
pub fn compare(options: &ComparisonOptions) -> Option<ComparisonResult> {
  let mut result = ComparisonResult::default();
  let file_1 = match File::open(&options.file_name_1) {
    Ok(file) => file,
    Err(reason) => {
      eprintln!("Unexpected: {:?}", reason);
      return None;
    }
  };
  let file_2 = match File::open(&options.file_name_2) {
    Ok(file) => file,
    Err(reason) => {
      eprintln!("Unexpected: {:?}", reason);
      return None;
    }
  };
  let buf_1 = BufReader::new(file_1);
  let buf_2 = BufReader::new(file_2);
  let mut iter_1 = buf_1.bytes().skip(options.skip_1).take(options.max_bytes);
  let mut iter_2 = buf_2.bytes().skip(options.skip_2).take(options.max_bytes);
  let mut first_difference = false;
  loop {
    match (iter_1.next(), iter_2.next()) {
      (Some(Ok(b1)), Some(Ok(b2))) => {
        if !first_difference && b1 == LF {
          result.first_difference_line += 1;
        }
        if result.size < options.marker_len {
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
          print_diff(result.size, Some(b1), Some(b2), options.verbose, options.byte_format);
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
        print_diff(result.size, None, Some(b2), options.verbose, options.byte_format);
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
        print_diff(result.size, Some(b1), None, options.verbose, options.byte_format);
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
