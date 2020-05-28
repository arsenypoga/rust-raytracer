pub mod point;
pub mod vector;
pub use point::{Point, ORIGIN};
use std::ops;
pub use vector::Vector;

pub trait Tuple: ops::Add + ops::Sub + ops::Mul<f64> + ops::Neg + Sized {
    fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self;
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
    fn get_w(&self) -> f64;
}
