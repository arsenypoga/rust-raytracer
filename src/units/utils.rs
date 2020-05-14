//! Miscelaneous utility functions

/// comparison constant
const EPSILON: f64 = 0.00001;

/// returns if both numbers are equal to an arbitrary number
/// called epsilon
///
/// # Arguments
/// * `a` - a float
/// * `b` - a float
///
/// # Examples
/// ```
/// use ::raytracer::units::utils;
/// let f = utils::float_eq(0.00001, 0.00002);
/// ```
pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
