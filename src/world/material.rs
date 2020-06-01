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

    // Returns a color of light at normal
    // pub fn lightning(
    //     &self,
    //     light: PointLight,
    //     position: Point,
    //     eyev: Vector,
    //     normalv: Vector,
    //     in_shadow: bool,
    // ) -> QuantColor {
    //     let intensity = QuantColor::new(
    //         light.intensity.r / 255,
    //         light.intensity.g / 255,
    //         light.intensity.b / 255,
    //     );
    //     let color = if self.pattern.is_some() {
    //         self.pattern.unwrap().color_at(position)
    //     } else {
    //         self.color
    //     };

    //     let effective_color = (color * intensity).clamp();
    //     let lightv = (light.position - position).normalize();

    //     let ambient = (effective_color * self.ambient as f64).clamp();
    //     let diffuse;
    //     let specular;

    //     let light_dot_normal = lightv.dot(normalv);
    //     if light_dot_normal < 0. {
    //         diffuse = BLACK;
    //         specular = BLACK;
    //     } else {
    //         diffuse = (effective_color * self.diffuse as f64 * light_dot_normal).clamp();
    //         let reflectv = (-lightv).reflect(normalv);
    //         let reflect_dot_eye = reflectv.dot(eyev);

    //         if reflect_dot_eye <= 0. {
    //             specular = BLACK;
    //         } else {
    //             let factor = reflect_dot_eye.powf(self.shine);
    //             specular = (light.intensity * self.specular as f64 * factor).clamp();
    //         }
    //     }
    //     if in_shadow {
    //         ambient
    //     } else {
    //         ambient + diffuse + specular
    //     }
    // }
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
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::units::color::WHITE;
//     use crate::units::tuple::Tuple;
//     use crate::world::light::PointLight;
// }
