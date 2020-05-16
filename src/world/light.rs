//! Light struct and methods

use crate::units::color::QuantColor;
use crate::units::tuple::Point;
/// Represents a point light
#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    /// How intense the light is
    pub intensity: QuantColor,
    /// Position of a light
    pub position: Point,
}

impl PointLight {
    /// Creates new light
    pub fn new(position: Point, intensity: QuantColor) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}
