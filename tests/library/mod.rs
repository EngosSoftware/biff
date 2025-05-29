use biff::{compare, ComparisonOptions, ComparisonResult};

#[test]
fn _0001() {
  let mut comparison_options = ComparisonOptions::default();
  comparison_options.skip_1 = 0;
  comparison_options.skip_2 = 0;
  comparison_options.max_bytes = usize::MAX;
  comparison_options.marker = vec![];
  comparison_options.percentage_limit = None;
  comparison_options.absolute_limit = None;

  let input_1: &[u8] = &[0, 1, 2, 3];
  let input_2: &[u8] = &[0, 1, 2, 3];
  assert_eq!(
    ComparisonResult::Identical,
    compare(input_1, input_2, &comparison_options)
  );
}
