//! Material struct and methods.

use crate::units::color::QuantColor;
// use crate::units::tuple::{Point, Vector};
// use crate::world::light::PointLight;
use crate::world::patterns::Pattern;
/// Represents a material
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    /// Material Color
    pub color: QuantColor,
    pub pattern: Option<Pattern>,
    /// Ambience level
    pub ambient: f64,
    /// Diffuse level
    pub diffuse: f64,
    /// Specular Level
    pub specular: f64,
    /// Shine level
    pub shine: f64,
    /// Reflect level. 1.0 is maximum reflection
    pub reflect: f64,
    /// transparency level 0.0 is default
    pub transparent: f64,
    /// refractive index 1.0 is default
    pub refractive_index: f64,
}

impl Material {
    /// Creates new default Material
    pub fn new(color: QuantColor) -> Material {
        Material {
            color,
            ..Material::default()
        }
    }

    pub fn set_pattern(&self, pattern: Option<Pattern>) -> Material {
        Material { pattern, ..*self }
    }

    pub fn set_ambient(&self, ambient: f64) -> Material {
        Material { ambient, ..*self }
    }

    pub fn set_color(&self, color: QuantColor) -> Material {
        Material { color, ..*self }
    }

    pub fn set_diffuse(&self, diffuse: f64) -> Material {
        Material { diffuse, ..*self }
    }
    pub fn set_shine(&self, shine: f64) -> Material {
        Material { shine, ..*self }
    }

    pub fn set_specular(&self, specular: f64) -> Material {
        Material { specular, ..*self }
    }
    pub fn set_reflect(&self, reflect: f64) -> Material {
        Material { reflect, ..*self }
    }

    pub fn set_transparency(&self, transparent: f64) -> Material {
        Material {
            transparent,
            ..*self
        }
    }

    pub fn set_refractive_index(&self, refractive_index: f64) -> Material {
        Material {
            refractive_index,
            ..*self
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: QuantColor::new(255, 255, 255),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shine: 200.,
            pattern: None,
            reflect: 0.,
            transparent: 0.,
            refractive_index: 1.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default() {
        let m = Material::default();
        assert_eq!(m.reflect, 0.);
        assert_eq!(m.transparent, 0.);
        assert_eq!(m.refractive_index, 1.);
    }
}
