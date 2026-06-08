/*
  Problem 15: Parse Integer with Result

  Write a function that takes a &str and attempts to parse it into an i32.
  Return Ok(value) on success, or Err(String) with a descriptive error message on failure.

  Run the tests for this problem with:
    cargo test --test parse_int_test
*/

pub fn parse_int(s: &str) -> Result<i32, String> {
    let x: Result<i32, _> = s.parse();
    match x {
      Ok(x) => Ok(x),
      Err(_) => Err(String::from("Error while parsing"))
    }
}
