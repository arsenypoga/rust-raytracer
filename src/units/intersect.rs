//! Intersection operations
use crate::units::Sphere;
// use std::ops;

/// Intersection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    /// Point of intersection
    pub t: f64,
    /// Intersected object
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    /// Creates new Intersection
    pub fn new(t: f64, object: &'a Sphere) -> Intersection {
        Intersection { t, object }
    }

    /// Returns hits from given intersection vector
    pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
        let i1 = xs[0];
        let i2 = xs[1];
        // let is_i1_positive =
        if (((i1.t > 0.) && (i2.t > 0.)) && (i1.t < i2.t)) || ((i1.t > 0.) && (i2.t < 0.)) {
            Some(i1)
        } else if (((i1.t > 0.) && (i2.t > 0.)) && (i1.t > i2.t)) || ((i1.t < 0.) && (i2.t > 0.)) {
            Some(i2)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
