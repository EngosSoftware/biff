use biff::{compare, ComparisonOptions, ComparisonResult};

#[test]
fn _0001() {
  let input_1: &[u8] = &[0, 1, 2, 3];
  let input_2: &[u8] = &[0, 1, 2, 3];
  assert_eq!(
    ComparisonResult::Identical,
    compare(input_1, input_2, &ComparisonOptions::default()),
  );
}

#[test]
fn _0002() {
  let input_1: &[u8] = &[0, 1, 2, 3];
  let input_2: &[u8] = &[0, 1, 2, 4];
  assert!(matches!(
    compare(input_1, input_2, &ComparisonOptions::default()),
    ComparisonResult::Different(_)
  ));
}

#[test]
fn _0003() {
  let input_1: &[u8] = &[0, 1, 2, 3];
  let input_2: &[u8] = &[0, 1, 2, 4];
  let mut options = ComparisonOptions::default();
  options.absolute_limit = Some(2);
  assert_eq!(
    compare(input_1, input_2, &options),
    ComparisonResult::SimilarAbsolute(2, 1)
  );
}

#[test]
fn _0004() {
  let input_1: &[u8] = &[0, 1, 2, 3];
  let input_2: &[u8] = &[0, 1, 2, 4];
  let mut options = ComparisonOptions::default();
  options.percentage_limit = Some(50.0);
  assert_eq!(
    compare(input_1, input_2, &options),
    ComparisonResult::SimilarPercentage(50.0, 25.0)
  );
}

#[test]
fn _0005() {
  let input_1: &[u8] = &[0, 1, 2, 3];
  let input_2: &[u8] = &[0, 1, 3, 4];
  let mut options = ComparisonOptions::default();
  options.absolute_limit = Some(1);
  assert_eq!(
    compare(input_1, input_2, &options),
    ComparisonResult::AbsoluteLimitExceeded(1, 2)
  );
}

#[test]
fn _0006() {
  let input_1: &[u8] = &[0, 1, 2, 3];
  let input_2: &[u8] = &[0, 1, 3, 4];
  let mut options = ComparisonOptions::default();
  options.percentage_limit = Some(25.0);
  assert_eq!(
    compare(input_1, input_2, &options),
    ComparisonResult::PercentageLimitExceeded(25.0, 50.0)
  );
}
