//! Material struct and methods.

use crate::units::color::{QuantColor, BLACK};
use crate::units::tuple::{Point, Vector};
use crate::world::light::PointLight;

/// Represents a material
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    /// Material Color
    pub color: QuantColor,
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
    pub fn new(
        color: QuantColor,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shine: f64,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shine,
        }
    }
    /// Returns a color of light at normal
    pub fn lightning(
        &self,
        light: PointLight,
        position: Point,
        eyev: Vector,
        normalv: Vector,
    ) -> QuantColor {
        let intensity = QuantColor::new(
            light.intensity.r / 255,
            light.intensity.g / 255,
            light.intensity.b / 255,
        );
        let effective_color = (self.color * intensity).clamp();
        let lightv = (light.position - position).normalize();

        let ambient = (effective_color * self.ambient as f64).clamp();
        let diffuse;
        let specular;

        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0. {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = (effective_color * self.diffuse as f64 * light_dot_normal).clamp();
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0. {
                specular = BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.shine);
                specular = (light.intensity * self.specular as f64 * factor).clamp();
            }
        }
        (ambient + diffuse + specular)
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::color::WHITE;
    use crate::units::tuple::Tuple;
    use crate::world::light::PointLight;

    #[test]
    fn lightning() {
        let m = Material::default();
        let p = Point::new(0, 0, 0);

        // Lighting with the eye between the light and the surface
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), QuantColor::new(255, 255, 255));
        let res = m.lightning(light, p, eyev, normalv);
        assert_eq!(res, QuantColor::new(483, 483, 483));

        // Lighting with the eye between light and surface, eye offset 45°
        let eyev = Vector::new(0., (2.0 as f64).sqrt() / 2., (2.0 as f64).sqrt() / 2.);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), WHITE);
        let res = m.lightning(light, p, eyev, normalv).clamp();
        assert_eq!(res, QuantColor::new(254, 254, 254));

        // Lighting with eye opposite surface, light offset 45°
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), QuantColor::new(255, 255, 255));
        let res = m.lightning(light, p, eyev, normalv);
        assert_eq!(res, QuantColor::new(186, 186, 186));

        // Lighting with eye in the path of the reflection vector
        let eyev = Vector::new(0., -(2.0 as f64).sqrt() / 2., -(2.0 as f64).sqrt() / 2.);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), QuantColor::new(255, 255, 255));
        let res = m.lightning(light, p, eyev, normalv);
        assert_eq!(res, QuantColor::new(415, 415, 415));

        // Lighting with the light behind the surface
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, 10), QuantColor::new(255, 255, 255));
        let res = m.lightning(light, p, eyev, normalv);
        assert_eq!(res, QuantColor::new(25, 25, 25));
    }
}
