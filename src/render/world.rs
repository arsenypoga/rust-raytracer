use crate::units::color::{QuantColor, BLACK, WHITE};
use crate::units::tuple::{Point, Tuple};
use crate::units::{Computations, Intersection, Matrix, Ray, Sphere};
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

    /// Compute world intersects
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();
        for o in &self.objects {
            intersections.extend(&ray.intersect(&o));
        }
        intersections.sort();
        intersections
    }

    /// Compute shading in the world.
    pub fn shade_hit(&self, c: Computations) -> QuantColor {
        c.object
            .material
            .lightning(self.light.unwrap(), c.point, c.eyev, c.normalv, false)
            .clamp()
    }

    /// Find color at a given ray
    pub fn color_at(&self, r: Ray) -> QuantColor {
        let intersections = self.intersect(r);
        let hits = Intersection::hit(intersections);
        match hits {
            Some(hit) => self.shade_hit(hit.computations(r)),
            None => BLACK,
        }
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
    use crate::units::tuple::Vector;
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

    #[test]
    fn shade_hit() {
        // Shading an intersection
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = w.objects[0];
        let i = Intersection::new(4., &shape);
        let comps = i.computations(r);
        let color = w.shade_hit(comps);
        assert_eq!(color, QuantColor::new(96, 120, 72));

        // Shading an intersection from the inside
        let mut w = World::default();
        let l = PointLight::new(Point::new(0., 0.25, 0.), QuantColor::new(255, 255, 255));
        w.light = Some(l);
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let comps = i.computations(r);
        let color = w.shade_hit(comps);
        assert_eq!(color, QuantColor::new(229, 229, 229));
    }

    #[test]
    fn color_at() {
        // The color when a ray misses
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));
        let c = w.color_at(r);
        assert_eq!(c, BLACK);

        // The color when a ray hits
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let c = w.color_at(r);
        println!("{:?}", c);

        assert_eq!(c, QuantColor::new(96, 120, 72));

        // The color with an intersection behind the ray
        let mut w = World::default();
        w.objects[0].material.ambient = 1.;
        w.objects[1].material.ambient = 1.;
        let inner = w.objects[1];
        let r = Ray::new(Point::new(0., 0., 0.75), Vector::new(0, 0, -1));
        let c = w.color_at(r);
        assert_eq!(c, inner.material.color);
    }
}
