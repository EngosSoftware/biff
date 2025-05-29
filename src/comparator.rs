use std::io::{BufReader, Read};

#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct ComparisonOptions {
  /// Number of starting bytes to skip in the first data stream.
  pub skip_1: usize,
  /// Number of starting bytes to skip in the second data stream.
  pub skip_2: usize,
  /// Maximum number of bytes to compare.
  pub max_bytes: usize,
  /// Expected file marker at the beginning of both data streams.
  pub marker: Vec<u8>,
  /// Accepted percentage limit of differences between compared data streams.
  pub percentage_limit: Option<f64>,
  /// Accepted absolute limit of differences between compared data streams.
  pub absolute_limit: Option<usize>,
}

/// Result details of file comparison.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ComparisonDetails {
  /// Size of the data, when data streams differ in sizes, the bigger size is returned.
  pub size: usize,
  /// Header marker of the first data stream.
  pub marker_1: Vec<u8>,
  /// Header marker of the second data stream.
  pub marker_2: Vec<u8>,
  /// Byte number of the first difference.
  pub first_difference_offset: usize,
  /// Line number of the first difference.
  pub first_difference_line: usize,
  /// First differing byte from the first data stream.
  pub first_difference_byte_1: Option<u8>,
  /// First differing byte from the second data stream.
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

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonResult {
  /// Compared data streams are identical.
  Identical,
  /// Compared data streams are similar, differences are under specified percentage limit.
  SimilarPercentage(f64, f64),
  /// Compared data streams are similar, differences are under specified absolute limit.
  SimilarAbsolute(usize, usize),
  /// Compared data streams are different.
  Different(ComparisonDetails),
  /// Percentage limit of differences was exceeded,
  /// contains the limit (first field) and actual difference (second field).
  PercentageLimitExceeded(f64, f64),
  /// Absolute limit of differences was exceeded,
  /// contains the limit (first field) and actual difference (second field).
  AbsoluteLimitExceeded(usize, usize),
  /// Invalid marker in the first data stream.
  InvalidMarker1(Vec<u8>, Vec<u8>),
  /// Invalid marker in the second data stream.
  InvalidMarker2(Vec<u8>, Vec<u8>),
  /// Comparison process ended with an unexpected error.
  Error(String),
}

/// Compares two files.
pub fn compare(reader_1: impl Read, reader_2: impl Read, options: &ComparisonOptions) -> ComparisonResult {
  let mut details = ComparisonDetails::default();
  let buf_1 = BufReader::new(reader_1);
  let buf_2 = BufReader::new(reader_2);
  let mut iter_1 = buf_1.bytes().skip(options.skip_1).take(options.max_bytes);
  let mut iter_2 = buf_2.bytes().skip(options.skip_2).take(options.max_bytes);
  let mut first_difference = false;
  loop {
    match (iter_1.next(), iter_2.next()) {
      (Some(Ok(byte_1)), Some(Ok(byte_2))) => {
        if !first_difference && byte_1 == b'\n' {
          details.first_difference_line += 1;
        }
        if details.size < options.marker.len() {
          details.marker_1.push(byte_1);
          details.marker_2.push(byte_2);
        }
        details.size += 1;
        if byte_1 != byte_2 {
          if !first_difference {
            details.first_difference_offset = details.size;
            details.first_difference_byte_1 = Some(byte_1);
            details.first_difference_byte_2 = Some(byte_2);
          }
          details.differences.push((details.size, Some(byte_1), Some(byte_2)));
          first_difference = true;
        }
      }
      (None, Some(Ok(byte_2))) => {
        details.size += 1;
        if !first_difference {
          details.first_difference_offset = details.size;
          details.first_difference_byte_2 = Some(byte_2);
        }
        details.differences.push((details.size, None, Some(byte_2)));
        first_difference = true;
      }
      (Some(Ok(byte_1)), None) => {
        if !first_difference && byte_1 == b'\n' {
          details.first_difference_line += 1;
        }
        details.size += 1;
        if !first_difference {
          details.first_difference_offset = details.size;
          details.first_difference_byte_1 = Some(byte_1);
        }
        details.differences.push((details.size, Some(byte_1), None));
        first_difference = true;
      }
      (None, None) => break,
      (value_1, value_2) => {
        return ComparisonResult::Error(format!("Reading bytes failed. {:?} {:?}", value_1, value_2))
      }
    }
  }
  if options.marker != details.marker_1 {
    return ComparisonResult::InvalidMarker1(options.marker.clone(), details.marker_1.clone());
  }
  if options.marker != details.marker_2 {
    return ComparisonResult::InvalidMarker2(options.marker.clone(), details.marker_2.clone());
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
