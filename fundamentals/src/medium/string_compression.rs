/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

pub fn compress(s: &str) -> String {
    let mut res_string: String = String::from("");
    if let Some(mut prev_char) = s.chars().nth(0) {
        let mut count = 0;
        for i in s.chars() {
            if i == prev_char {
                count = count + 1;
            } else {
                res_string.push(prev_char);
                res_string = res_string + &count.to_string();
                prev_char = i;
                count = 1;
            }
        }
        res_string.push(prev_char);
        res_string = res_string + &count.to_string();
        if res_string.len() >= s.len() {
            return s.to_string();
        } else {
            return res_string;
        }
    } else {
        return String::from("");
    }
}
