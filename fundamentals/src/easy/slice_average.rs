/*
  Problem 11: Slice Average

  Write a function that takes a slice of f64 values and returns the arithmetic mean.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test slice_average_test
*/

pub fn average(values: &[f64]) -> Option<f64> {
    if values.len() < 1 {
      None
    }
    else {
      let mean:f64 ;
      let mut count = 0.00;
      let mut sum = 0.0;
      for value in values {
        sum += *value;
        count += 1.0;
      }
      mean = sum / count;
      Some(mean)
    }
}
