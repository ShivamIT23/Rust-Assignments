/*
  Problem 31: Fibonacci Iterator

  Implement a struct Fibonacci that implements the Iterator trait.
  The iterator should yield successive Fibonacci numbers starting from 0, 1, 1, 2, 3, 5, ...
  Use u64 to allow for larger numbers.

  Run the tests for this problem with:
    cargo test --test fibonacci_iter_test
*/

pub struct Fibonacci {
    pub a: u64,
    pub b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a + self.b;
        let send_item = self.a;
        self.a = self.b;
        self.b = c;
        Some(send_item)
    }
}
