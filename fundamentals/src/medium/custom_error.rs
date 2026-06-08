/*
  Problem 26: Custom Error Type

  Define a custom error enum ValidationError with variants TooShort, TooLong,
  and InvalidChar(char). Write a function that validates a username:
  must be 3–20 characters and only contain alphanumeric chars or underscores.
  Return Ok(()) or the appropriate error.

  Run the tests for this problem with:
    cargo test --test custom_error_test
*/

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort,
    TooLong,
    InvalidChar(char),
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let size = username.len();
    for char in username.chars() {
        if !char.is_alphabetic() && !(char == '_') {
            return Err(ValidationError::InvalidChar(char));
        }
    }
    if size < 3 {
        Err(ValidationError::TooShort)
    }
    else if size > 20 {
        Err(ValidationError::TooLong)
    }
    else{
        Ok(())
    }
}
