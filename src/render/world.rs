use crate::units::color::{QuantColor, BLACK, WHITE};
use crate::units::tuple::{Point, Tuple, Vector};
use crate::units::{Computations, Intersection, Matrix, Ray};

use crate::units::objects::Shape;
use crate::world::{Material, PointLight};
pub struct World {
    /// vector of objects in the world.
    pub objects: Vec<Shape>,
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
            intersections.extend(o.intersect(ray));
        }
        intersections.sort();
        intersections
    }

    /// Compute shading in the world.
    pub fn shade_hit(&self, c: Computations, remaining: usize) -> QuantColor {
        let base_color = c.object.lightning(
            self.light.unwrap(),
            c.over_point,
            c.eyev,
            c.normalv,
            self.is_shadowed(c.over_point),
        );

        let reflect_color = self.reflect_color(c, remaining);
        (base_color + reflect_color).clamp()
    }

    /// Find color at a given ray
    pub fn color_at(&self, r: Ray, remaining: usize) -> QuantColor {
        let intersections = self.intersect(r);
        let hits = Intersection::hit(intersections);
        match hits {
            Some(hit) => self.shade_hit(hit.computations(r), remaining),
            None => BLACK,
        }
    }
    pub fn reflect_color(&self, comps: Computations, remaining: usize) -> QuantColor {
        if comps.object.material.reflect == 0. || remaining == 0 {
            BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let color = self.color_at(reflect_ray, remaining - 1);
            color * comps.object.material.reflect
        }
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let v = match self.light {
            Some(l) => l.position - point,
            None => Vector::new(0, 0, 0),
        };

        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);
        let hit = Intersection::hit(intersections);

        hit.is_some() && hit.unwrap().t < distance
    }

    pub fn set_light(&self, light: Option<PointLight>) -> World {
        World {
            objects: self.objects.to_owned(),
            light,
        }
    }

    pub fn set_objects(&self, objects: Vec<Shape>) -> World {
        World {
            objects,
            light: self.light,
        }
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight::new(Point::new(-10, 10, -10), WHITE);
        let s1 = Shape {
            material: Material {
                color: QuantColor::new(204, 255, 153),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
            ..Shape::default()
        };

        let s2 = Shape {
            transformation_matrix: Matrix::scale(0.5, 0.5, 0.5),
            ..Shape::default()
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
    use crate::units::objects::ObjectType;
    use crate::units::{tuple::Vector, Transformable};

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
        let color = w.shade_hit(comps, 1);
        assert_eq!(color, QuantColor::new(96, 120, 72));

        // Shading an intersection from the inside
        let mut w = World::default();
        let l = PointLight::new(Point::new(0., 0.25, 0.), WHITE);
        w.light = Some(l);
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let comps = i.computations(r);
        let color = w.shade_hit(comps, 1);
        assert_eq!(color, QuantColor::new(229, 229, 229));

        // shade_hit() is given an intersection in shadow
        let mut w = World::new();
        let l = PointLight::new(Point::new(0, 0, -10), WHITE);
        w.light = Some(l);
        let mut s1 = Shape::new(ObjectType::Sphere);
        s1.transformation_matrix = Matrix::translate(0, 0, 10);
        w.objects = vec![Shape::new(ObjectType::Sphere), s1];

        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let i = Intersection::new(0.5, &s1);
        let comps = i.computations(r);
        let color = w.shade_hit(comps, 1);
        assert_eq!(color, QuantColor::new(25, 25, 25));

        // shade_hit() with a reflective material
        let mut w = World::default();
        let s = Shape::new(ObjectType::Plane)
            .set_material(Material::default().set_reflect(0.5))
            .translate(0, -1, 0);
        w.objects.push(s);
        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0., -(2_f64).sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new(2_f64.sqrt(), &s);
        let comps = i.computations(r);
        let color = w.shade_hit(comps, 1);
        assert_eq!(QuantColor::new(222, 234, 210), color);
    }

    #[test]
    fn color_at() {
        // The color when a ray misses
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));
        let c = w.color_at(r, 1);
        assert_eq!(c, BLACK);

        // The color when a ray hits
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let c = w.color_at(r, 1);
        println!("{:?}", c);

        assert_eq!(c, QuantColor::new(96, 120, 72));

        // The color with an intersection behind the ray
        let mut w = World::default();
        w.objects[0].material.ambient = 1.;
        w.objects[1].material.ambient = 1.;
        let inner = w.objects[1];
        let r = Ray::new(Point::new(0., 0., 0.75), Vector::new(0, 0, -1));
        let c = w.color_at(r, 1);
        assert_eq!(c, inner.material.color);

        // color_at() with mutually reflective surfaces
        let mut w = World::new().set_light(Some(PointLight::new(
            Point::new(0, 0, 0),
            QuantColor::new(255, 255, 255),
        )));

        let lower = Shape::new(ObjectType::Plane)
            .set_material(Material::default().set_reflect(1.))
            .translate(0, -1, 0);

        let upper = Shape::new(ObjectType::Plane)
            .set_material(Material::default().set_reflect(1.))
            .translate(0, 1, 0);

        w.objects = vec![lower, upper];
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 1, 0));
        let color = w.color_at(r, 1);
        println!("{:?}", color);
    }

    #[test]
    fn is_shadowed() {
        // There is no shadow when nothing is collinear with point and light
        let w = World::default();
        let p = Point::new(0, 10, 0);
        assert!(!w.is_shadowed(p));

        // The shadow when an object is between the point and the light
        let w = World::default();
        let p = Point::new(10, -10, 10);
        assert!(w.is_shadowed(p));

        // There is no shadow when an object is behind the light
        let w = World::default();
        let p = Point::new(-20, 20, -20);
        assert!(!w.is_shadowed(p));

        // There is no shadow when an object is behind the point
        let w = World::default();
        let p = Point::new(-2, 2, -2);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn reflect_color() {
        // The reflected color for a nonreflective material
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let mut shape: Shape = w.objects[1];
        shape.material.ambient = 1.;
        let i = Intersection::new(1., &shape);
        let comps = i.computations(r);
        let color = w.reflect_color(comps, 1);
        assert_eq!(color, BLACK);

        // The reflected color for a reflective material
        let mut w = World::default();
        let shape = Shape::new(ObjectType::Plane)
            .set_material(Material::default().set_reflect(0.5))
            .translate(0, -1, 0);

        w.objects.push(shape);
        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0., -(2_f64).sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new(2_f64.sqrt(), &shape);
        let comps = i.computations(r);
        let color = w.reflect_color(comps, 1);
        assert_eq!(QuantColor::new(48, 60, 36), color);

        // The reflected color at the maximum recursive depth
        let mut w = World::default();
        let shape = Shape::new(ObjectType::Plane)
            .set_material(Material::default().set_reflect(0.5))
            .translate(0, -1, 0);
        w.objects.push(shape);
        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0., -(2_f64).sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let int = Intersection::new(2_f64.sqrt(), &shape);
        let comps = i.computations(r);
        let color = w.reflect_color(comps, 0);
        assert_eq!(BLACK, color);
    }
}
