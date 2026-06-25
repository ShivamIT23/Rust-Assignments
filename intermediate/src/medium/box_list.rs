/*
  Problem 39: Box<T> — Recursive List

  Define a recursive List enum using Box<T> with variants Cons(i32, Box<List>) and Nil.
  Implement a method sum() that returns the sum of all elements,
  and a method len() that returns the number of elements.

  Run the tests for this problem with:
    cargo test --test box_list_test
*/

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn sum(&self) -> i32 {
        let mut sum = 0;
        match self {
            List::Nil => return 0,
            List::Cons(v, k) => {
                sum = sum + v + k.sum();
            }
        }
        sum
    }

    pub fn len(&self) -> usize {
        match self {
            List::Nil => return 0,
            List::Cons(v, k) => {
                1 + k.len()
            }
        }
    }
}
