/*
  Problem 32: HashMap — Group By First Letter

  Write a function that takes a Vec<String> and returns a HashMap<char, Vec<String>>
  where words are grouped by their first character (lowercase).
  Ignore empty strings.

  Run the tests for this problem with:
    cargo test --test hashmap_groupby_test
*/

use std::collections::HashMap;

pub fn group_by_first_letter(words: Vec<String>) -> HashMap<char, Vec<String>> {
  let mut vec_hash : HashMap<char, Vec<String>>  = HashMap::new();
    for word in words{
      if let Some(char) = word.chars().nth(0) {
        vec_hash.entry(char).or_default().push(word);
      }
    }
    vec_hash
}
