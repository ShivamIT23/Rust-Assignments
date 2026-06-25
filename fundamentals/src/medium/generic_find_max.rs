/*
  Problem 28: Generic — Find Max

  Write a generic function that takes a slice of items implementing PartialOrd
  and returns Option<&T> for the maximum element.
  Do not use any built-in max functions.

  Run the tests for this problem with:
    cargo test --test generic_find_max_test
*/

pub fn find_max<T: PartialOrd>(items: &[T]) -> Option<&T> {
    if items.len() == 0 {
        return None;
    } else {
        let mut max: &T = items.iter().nth(0).unwrap();
        for i in items.iter() {
            if max < i {
                max = i
            }
        }
        Some(max)
    }
}
