#[macro_use]
extern crate lazy_static;

use std::ops::Mul;

pub fn square<T: Mul<Output=T> + Copy>(x: T) -> T {
    x * x
}
mod complex;
pub use complex::C;

mod canvas;
pub use canvas::Canvas;

pub mod colormap;
