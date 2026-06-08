/*
  Problem 19: Word Counter

  Write a function that takes a &str and returns a HashMap<String, usize>
  where each key is a lowercase word and each value is the number of occurrences.
  Split on whitespace and convert to lowercase.

  Run the tests for this problem with:
    cargo test --test word_counter_test
*/

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut x: HashMap<String, usize> = HashMap::new();
    if text.len() == 0 {return x;}
    for i in text.rsplit(" ") {
      let count : usize;
      match x.get(&i.to_string().to_ascii_lowercase()) {
        Some(x) => count = x + 1,
        None => count = 1,
      }
      x.insert(i.to_string().to_ascii_lowercase(), count);
    }
    x
}
