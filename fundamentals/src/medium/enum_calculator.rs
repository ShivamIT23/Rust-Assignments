/*
  Problem 21: Enum Calculator

  Define an enum Operation with variants Add(f64, f64), Sub(f64, f64), Mul(f64, f64), and Div(f64, f64).
  Write a function that takes an Operation and returns Result<f64, String>,
  returning an error for division by zero.

  Run the tests for this problem with:
    cargo test --test enum_calculator_test
*/

pub enum Operation {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

pub fn calculate(op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add(x, y) => Ok(x+y),
        Operation::Sub(x, y) => Ok(x-y),
        Operation::Mul(x, y) => Ok(x*y),
        Operation::Div(x, y) => {
            if y == 0.0 {
                Err(String::from("Divisor cannot be zero"))
            }
             else { 
                Ok(x/y)
            }
        },
    }
}
