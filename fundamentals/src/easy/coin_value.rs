/*
  Problem 22: Pattern Matching — Coin Value

  Define an enum Coin with variants Penny, Nickel, Dime, and Quarter.
  Write a function that returns the value in cents as u32 using pattern matching.

  Run the tests for this problem with:
    cargo test --test coin_value_test
*/

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn coin_value(coin: Coin) -> u32 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
