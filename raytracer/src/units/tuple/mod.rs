pub mod point;
pub mod vector;
pub use point::{Point, ORIGIN};
use std::ops;
pub use vector::Vector;

/// Describes a 4 field data structure with x, y, z, w fields.
pub trait Tuple: ops::Add + ops::Sub + ops::Mul<f64> + ops::Neg + Sized {
    /// Creates new Object of type Tuple
    fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self;
    /// accesses x value
    fn get_x(&self) -> f64;
    /// accesses y value
    fn get_y(&self) -> f64;
    /// accesses z value
    fn get_z(&self) -> f64;
    /// accesses w value
    fn get_w(&self) -> f64;
}
