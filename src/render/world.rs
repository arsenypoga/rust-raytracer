use crate::units::color::{QuantColor, WHITE};
use crate::units::tuple::{Point, Tuple, Vector};
use crate::units::{Intersection, Matrix, Ray, Sphere};
use crate::world::{Material, PointLight};
pub struct World {
    /// vector of objects in the world.
    pub objects: Vec<Sphere>,
    /// World light
    pub light: Option<PointLight>,
}

impl World {
    /// Creates new world with no objects and no light source
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();
        for o in &self.objects {
            intersections.extend(&ray.intersect(&o));
        }
        intersections.sort();
        intersections
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight::new(Point::new(-10, 10, -10), WHITE);
        let s1 = Sphere {
            material: Material {
                color: QuantColor::new(204, 255, 153),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
            ..Sphere::default()
        };

        let s2 = Sphere {
            transform_matrix: Matrix::scale(0.5, 0.5, 0.5),
            ..Sphere::default()
        };

        World {
            light: Some(light),
            objects: vec![s1, s2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert!(w.light.is_none());
    }
    #[test]
    fn intersect() {
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let ints = w.intersect(r);
        assert_eq!(ints.len(), 4);
        println!("{:?}", ints);
    }
}
