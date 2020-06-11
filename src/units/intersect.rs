//! Intersection operations
use crate::units::objects::Shape;
use crate::units::tuple::{Point, Vector};
use crate::units::utils;
use crate::units::Ray;
use std::cmp::Ordering;
/// Intersection
#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    /// Point of intersection
    pub t: f64,
    /// Intersected object
    pub object: &'a Shape,
}

pub struct Computations<'a> {
    /// T of intersection for the ray
    pub t: f64,
    /// Object of computations
    pub object: &'a Shape,
    /// Point of intersections
    pub point: Point,
    /// Point for approximation
    pub over_point: Point,
    /// Eye vector
    pub eyev: Vector,
    /// Normal vector at the point
    pub normalv: Vector,
    /// Reflection vector at the point
    pub reflectv: Vector,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    /// Creates new Intersection
    pub fn new(t: f64, object: &'a Shape) -> Intersection<'a> {
        Intersection { t, object }
    }

    /// Returns hits from given intersection vector
    pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
        let mut clone = xs.clone();
        clone.retain(|i| i.t > 0.);
        clone.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        if clone.is_empty() {
            None
        } else {
            Some(clone[0])
        }
    }

    pub fn computations(&self, r: Ray) -> Computations {
        let position = r.position(self.t);
        let mut normalv = self.object.normal(position);
        let eyev = -r.direction;
        let inside = normalv.dot(eyev) < 0.;

        if normalv.dot(eyev) < 0. {
            normalv = -normalv;
        }

        Computations {
            t: self.t,
            object: self.object,
            point: position,
            eyev: -r.direction,
            normalv,
            inside,
            over_point: position + normalv * utils::EPSILON,
            reflectv: r.direction.reflect(normalv),
        }
    }
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.t - other.t) > utils::EPSILON {
            Ordering::Greater
        } else if (self.t - other.t) < utils::EPSILON {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Intersection) -> bool {
        utils::float_eq(self.t, other.t)
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        if (self.t - other.t) > utils::EPSILON {
            Some(Ordering::Greater)
        } else if (self.t - other.t) < utils::EPSILON {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::objects::ObjectType;
    use crate::units::tuple::Tuple;
    use crate::units::Matrix;
    #[test]
    fn hit() {
        // The hit, when all intersections have positive t
        let s = Shape::new(ObjectType::Sphere);
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let xs = vec![i1, i2];

        let x = Intersection::hit(xs).unwrap();
        assert_eq!(x, i1);

        // The hit, when some intersections have negative t
        let s = Shape::new(ObjectType::Sphere);

        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let xs = vec![i1, i2];

        let x = Intersection::hit(xs).unwrap();
        assert_eq!(x, i2);

        // The hit, when all intersections have negative t
        let s = Shape::new(ObjectType::Sphere);
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(-2., &s);
        let xs = vec![i1, i2];

        assert!(Intersection::hit(xs).is_none());
    }

    #[test]
    fn computations() {
        // The hit, when an intersection occurs on the outside
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Shape::new(ObjectType::Sphere);
        let i = Intersection::new(4., &shape);
        let comps = i.computations(r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, -1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
        assert!(!comps.inside);

        // The hit, when an intersection occurs on the inside
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = Shape::new(ObjectType::Sphere);
        let i = Intersection::new(1., &shape);
        let comps = i.computations(r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, 1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
        assert!(comps.inside);

        // The hit should offset the point
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut shape = Shape::new(ObjectType::Sphere);

        shape.transformation_matrix = Matrix::translate(0, 0, 1);
        let i = Intersection::new(5., &shape);
        let comps = i.computations(r);
        assert!(comps.over_point.z < -utils::EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);

        // Precomputing the reflection vector
        let s = Shape::new(ObjectType::Plane);
        let r = Ray::new(
            Point::new(0, 1, -1),
            Vector::new(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new(2_f64.sqrt(), &s);
        let comps = i.computations(r);
        assert_eq!(
            comps.reflectv,
            Vector::new(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
        );
    }
}
