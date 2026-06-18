// use std::f32::consts::PI;
// use std::fmt::Display;

// trait Shape{
//     fn area(&self) -> f32;
//     fn perimeter(&self) -> f32;
// }

// use std::fmt::{Result};

// #[derive(Debug)]
// struct Rect {
//     width : f32,
//     height : f32,
// }

// #[derive()]
// struct Circle {
//     radius : f32,
// }

// impl Display for Rect {
// //     fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // let width = self.width;
//         // let height = self.height;
//         // println!("Width is {} and height is {}",width,height);
//         // return Ok(());
// //     }

//         // Real implementation
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
//             write!(f, "Rectangle has a height = {}, and a width = {}",self.height,self.width)
//         }
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height;
//     }
//     fn perimeter(&self) -> f32 {
//         return 2.0 * (self.width + self.height);
//     }
// }

// impl Shape for Circle {
//     fn area(&self) -> f32 {
//         return PI * self.radius * self.radius;
//     }
//     fn perimeter(&self) -> f32 {
//         return 2.0 * PI * self.radius;
//     }
// }

// fn main() {
//     let r = Rect {
//         width : 10.0,
//         height : 20.0
//     };
//     let c = Circle { radius: 30.0 };

//     println!("{}",r);

//     println!("{}",get_perimeter_and_area(r).0);
//     println!("{}",get_perimeter_and_area(c).0);
// }

// // fn get_perimeter_and_area(s: impl Shape) -> (f32,f32) {
// //     return (s.area(),s.perimeter())
// // }

// fn get_perimeter_and_area<T :Shape>(s: T) -> (f32,f32) {
//     return (s.area(),s.perimeter())
// }

// macro_rules! say_hello {
//     () => {
//         println!("Hello World");
//     }
// }

// macro_rules! create_function {
//     ($func_name:ident) => {
//         fn $func_name() {
//             println!("Hello from {}",stringify!($func_name));
//         }
//     }
// }

// create_function!(hii);

fn main() {
    // let a = 1;
    // say_hello!();
    // hii();
    // let r = Rect {
    //     width : 10.0,
    //     height : 20.0
    // };
    // println!("{}",r);
    // println!("{:?}",r);
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let v1_iter = v1_iter.map(|x| *x + 1);
    let sum: i32 = v1_iter.sum();

    println!("{}", sum);
    println!("{:?}", v1);
}
// 4:07:28 in yt rust marathon in 6 hours.