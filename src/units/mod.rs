pub mod color;
pub mod intersect;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod tuple;
pub mod utils;

pub use intersect::{Computations, Intersection};
pub use matrix::{Matrix, IDENTITY_MATRIX};
pub use ray::Ray;
pub use sphere::Sphere;
