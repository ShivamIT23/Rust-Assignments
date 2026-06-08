/*
  Problem 13: Color Enum

  Define an enum Color with variants Red, Green, Blue, and Custom(u8, u8, u8).
  Implement a function that converts a Color to its RGB tuple (u8, u8, u8).

  Run the tests for this problem with:
    cargo test --test color_enum_test
*/

pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

pub fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Blue => (0,0,255),
        Color::Green => (0,255,0),
        Color::Red => (255,0,0),
        Color::Custom(u, v, w) => (u,v,w)
    }
}
