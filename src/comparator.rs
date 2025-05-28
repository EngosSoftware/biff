use std::fs::File;
use std::io::{BufReader, Read};

const LF: u8 = b'\n';

#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct ComparisonOptions {
  /// Name of the first file in comparison.
  pub file_name_1: String,
  /// Number of starting bytes to skip in the first file.
  pub skip_1: usize,
  /// Name of the second file in comparison.
  pub file_name_2: String,
  /// Number of starting bytes to skip in the second file.
  pub skip_2: usize,
  /// Maximum number of bytes to compare.
  pub max_bytes: usize,
  /// Expected file marker at the beginning of both files.
  pub marker: Vec<u8>,
  /// Accepted percentage limit of differences between compared files.
  pub percentage_limit: Option<f64>,
  /// Accepted absolute limit of differences between compared files.
  pub absolute_limit: Option<usize>,
}

/// Result details of file comparison.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ComparisonDetails {
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
  /// List of different bytes with offsets.
  pub differences: Vec<(usize, Option<u8>, Option<u8>)>,
}

impl Default for ComparisonDetails {
  fn default() -> Self {
    Self::new()
  }
}

impl ComparisonDetails {
  /// Creates new comparison result.
  fn new() -> Self {
    Self {
      size: 0,
      marker_1: vec![],
      marker_2: vec![],
      first_difference_offset: 0,
      first_difference_line: 1,
      first_difference_byte_1: None,
      first_difference_byte_2: None,
      differences: vec![],
    }
  }
}

pub enum ComparisonResult {
  /// Compared files are identical.
  Identical,
  /// Compared files are similar, differences are under specified percentage limit.
  SimilarPercentage(f64, f64),
  /// Compared files are similar, differences are under specified absolute limit.
  SimilarAbsolute(usize, usize),
  /// Compared files are different.
  Different(ComparisonDetails),
  /// Percentage limit of differences was exceeded,
  /// contains the limit (first field) and actual difference (second field).
  PercentageLimitExceeded(f64, f64),
  /// Absolute limit of differences was exceeded,
  /// contains the limit (first field) and actual difference (second field).
  AbsoluteLimitExceeded(usize, usize),
  /// File has and invalid marker.
  InvalidMarker(String, Vec<u8>, Vec<u8>),
  /// Comparison process ended with an error.
  Error(String),
}

/// Compares two files.
pub fn compare(options: &ComparisonOptions) -> ComparisonResult {
  let mut details = ComparisonDetails::default();
  let file_1 = match File::open(&options.file_name_1) {
    Ok(file) => file,
    Err(reason) => return ComparisonResult::Error(format!("Can not open file. {:?}", reason)),
  };
  let file_2 = match File::open(&options.file_name_2) {
    Ok(file) => file,
    Err(reason) => return ComparisonResult::Error(format!("Can not open file. {:?}", reason)),
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
          details.first_difference_line += 1;
        }
        if details.size < options.marker.len() {
          details.marker_1.push(b1);
          details.marker_2.push(b2);
        }
        details.size += 1;
        if b1 != b2 {
          if !first_difference {
            details.first_difference_offset = details.size;
            details.first_difference_byte_1 = Some(b1);
            details.first_difference_byte_2 = Some(b2);
          }
          details.differences.push((details.size, Some(b1), Some(b2)));
          first_difference = true;
        }
      }
      (None, Some(Ok(b2))) => {
        details.size += 1;
        if !first_difference {
          details.first_difference_offset = details.size;
          details.first_difference_byte_2 = Some(b2);
        }
        details.differences.push((details.size, None, Some(b2)));
        first_difference = true;
      }
      (Some(Ok(b1)), None) => {
        if !first_difference && b1 == LF {
          details.first_difference_line += 1;
        }
        details.size += 1;
        if !first_difference {
          details.first_difference_offset = details.size;
          details.first_difference_byte_1 = Some(b1);
        }
        details.differences.push((details.size, Some(b1), None));
        first_difference = true;
      }
      (None, None) => break,
      (v1, v2) => return ComparisonResult::Error(format!("Unexpected: {:?}, {:?}", v1, v2)),
    }
  }
  if options.marker != details.marker_1 {
    return ComparisonResult::InvalidMarker(
      options.file_name_1.clone(),
      options.marker.clone(),
      details.marker_1.clone(),
    );
  }
  if options.marker != details.marker_2 {
    return ComparisonResult::InvalidMarker(
      options.file_name_2.clone(),
      options.marker.clone(),
      details.marker_2.clone(),
    );
  }
  let absolute_difference = details.differences.len();
  if let Some(limit) = options.percentage_limit {
    let percentage_difference = absolute_difference as f64 * 100.0 / details.size as f64;
    return if percentage_difference > limit {
      ComparisonResult::PercentageLimitExceeded(limit, percentage_difference)
    } else {
      ComparisonResult::SimilarPercentage(limit, percentage_difference)
    };
  }
  if let Some(limit) = options.absolute_limit {
    return if absolute_difference > limit {
      ComparisonResult::AbsoluteLimitExceeded(limit, absolute_difference)
    } else {
      ComparisonResult::SimilarAbsolute(limit, absolute_difference)
    };
  }
  if absolute_difference > 0 {
    return ComparisonResult::Different(details);
  }
  ComparisonResult::Identical
}
