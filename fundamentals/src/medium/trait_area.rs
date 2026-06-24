/*
  Problem 27: Trait — Area Calculation

  Define a trait Shape with a method fn area(&self) -> f64.
  Implement it for structs Circle { radius: f64 } and Square { side: f64 }.
  Write a function print_area that accepts any &dyn Shape and returns the area.

  Run the tests for this problem with:
    cargo test --test trait_area_test
*/

use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius *self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn print_area(shape: &dyn Shape) -> f64 {
    shape.area()
}
