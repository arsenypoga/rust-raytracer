//! Intersection operations
use crate::units::tuple::{Point, Vector};
use crate::units::utils;
use crate::units::Ray;
use crate::units::Sphere;
use std::cmp::Ordering;

/// Intersection
#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    /// Point of intersection
    pub t: f64,
    /// Intersected object
    pub object: &'a Sphere,
}

pub struct Computations<'a> {
    /// T of intersection for the ray
    pub t: f64,
    /// Object of computations
    pub object: &'a Sphere,
    /// Point of intersections
    pub point: Point,
    /// Eye vector
    pub eyev: Vector,
    /// Normal vector at the point
    pub normalv: Vector,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    /// Creates new Intersection
    pub fn new(t: f64, object: &'a Sphere) -> Intersection {
        Intersection { t, object }
    }

    /// Returns hits from given intersection vector
    pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
        if xs.is_empty() {
            None
        } else if xs.len() == 1 {
            Some(xs[0])
        } else {
            let i1 = xs[0];
            let i2 = xs[1];
            // let is_i1_positive =
            if (((i1.t > 0.) && (i2.t > 0.)) && (i1.t < i2.t)) || ((i1.t > 0.) && (i2.t < 0.)) {
                Some(i1)
            } else if (((i1.t > 0.) && (i2.t > 0.)) && (i1.t > i2.t))
                || ((i1.t < 0.) && (i2.t > 0.))
            {
                Some(i2)
            } else {
                None
            }
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
    use crate::units::tuple::Tuple;
    #[test]
    fn hit() {
        // The hit, when all intersections have positive t
        let s = Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let xs = vec![i1, i2];

        let x = Intersection::hit(xs).unwrap();
        assert_eq!(x, i1);

        // The hit, when some intersections have negative t
        let s = Sphere::new();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let xs = vec![i1, i2];

        let x = Intersection::hit(xs).unwrap();
        assert_eq!(x, i2);

        // The hit, when all intersections have negative t
        let s = Sphere::new();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(-2., &s);
        let xs = vec![i1, i2];

        assert!(Intersection::hit(xs).is_none());
    }

    #[test]
    fn computations() {
        // The hit, when an intersection occurs on the outside
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();
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
        let shape = Sphere::default();
        let i = Intersection::new(1., &shape);
        let comps = i.computations(r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, 1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
        assert!(comps.inside);
    }
}
