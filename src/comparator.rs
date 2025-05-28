use crate::{print_byte, ByteFormat};
use std::fs::File;
use std::io::{BufReader, Read};

const LF: u8 = b'\n';

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

/// Result details of file comparison.
pub struct ComparisonResult {
  /// Total number of different bytes in compared files.
  counter: usize,
  /// Size of the files, when files differ in sizes, the bigger file size is returned.
  size: usize,
  /// Header marker of the first file.
  marker_1: Vec<u8>,
  /// Header marker of the second file.
  marker_2: Vec<u8>,
  /// Byte number of the first difference.
  first_difference_offset: usize,
  /// Line number of the first difference.
  first_difference_line: usize,
  /// First differing byte from the first file.
  first_difference_byte_1: Option<u8>,
  /// First differing byte from the second file.
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

  /// Returns the total number of different bytes in compared files.
  pub fn counter(&self) -> usize {
    self.counter
  }

  /// Returns the size of the files, when files differ in sizes,
  /// the bigger file size is returned.
  pub fn size(&self) -> usize {
    self.size
  }

  /// Returns the header marker of the first file.
  pub fn marker_1(&self) -> &[u8] {
    &self.marker_1
  }

  /// Returns the header marker of the second file.
  pub fn marker_2(&self) -> &[u8] {
    &self.marker_2
  }

  /// Returns the byte number of the first difference.
  pub fn first_difference_offset(&self) -> usize {
    self.first_difference_offset
  }

  /// Returns the line number of the first difference.
  pub fn first_difference_line(&self) -> usize {
    self.first_difference_line
  }

  /// Returns the first differing byte from the first file.
  pub fn first_difference_byte_1(&self) -> Option<u8> {
    self.first_difference_byte_1
  }

  /// Returns the first differing byte from the second file.
  pub fn first_difference_byte_2(&self) -> Option<u8> {
    self.first_difference_byte_2
  }
}

/// Compares two files.
#[allow(clippy::too_many_arguments)]
pub fn compare(
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
