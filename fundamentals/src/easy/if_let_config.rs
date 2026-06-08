/*
  Problem 23: If Let — Extract Config Value

  Write a function that takes an Option<String> representing a config value.
  If it is Some, return the inner value. If None, return "default" as a String.
  Use if let or unwrap_or.

  Run the tests for this problem with:
    cargo test --test if_let_config_test
*/

pub fn get_config(value: Option<String>) -> String {
    if let Some(x) = Some(value.unwrap_or(String::from("default"))) {
        return x;
    }
    else {
        return String::from("default");
    }
}
