pub mod color;
pub mod intersect;
pub mod matrix;
pub mod objects;
pub mod ray;
pub mod tuple;
pub mod utils;

pub use intersect::{Computations, Intersection};
pub use matrix::{Matrix, Transformable, IDENTITY_MATRIX};
pub use ray::Ray;
