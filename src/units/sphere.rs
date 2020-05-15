//! All sphere operations are defined here

use crate::units::tuple::{Point, Tuple, Vector};
use crate::units::{Matrix, IDENTITY_MATRIX};

// pub trait Object {}

///Sphere represents a spere object.
/// With no data for now
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    /// Origin point of a sphere, where it's centered.
    pub origin: Point,
    /// Radius of a sphere
    pub radius: f64,
    /// Transform matrix for an object
    pub transform_matrix: Matrix,
}

impl Sphere {
    /// Creates a new sphere at origin
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::Sphere;
    ///
    /// let s = Sphere::new();
    ///
    /// ```
    pub fn new() -> Sphere {
        Sphere {
            origin: Point::new(0, 0, 0),
            radius: 1.0,
            transform_matrix: IDENTITY_MATRIX,
        }
    }

    /// Transforms sphere with the new transformation matrix
    ///
    /// # Arguments
    /// * `transform` - a new transformation matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::{Sphere, Matrix};
    ///
    /// let s = Sphere::new();
    /// let m = Matrix::rotate_z(1.);
    /// let s1 = s.transform(m);
    ///
    /// ```
    pub fn transform(&self, transform: Matrix) -> Sphere {
        Sphere {
            origin: self.origin,
            radius: 1.0,
            transform_matrix: transform,
        }
    }
    /// Computes normal at the world point of an object
    ///
    /// # Arguments
    /// * `world_point` - a point with world  coordinates that computes normal vector
    ///
    /// # Examples
    ///
    /// use ::raytracer::units::{Sphere, Tuple};
    ///
    /// let s = Sphere::new();
    /// let n = s.normal(Point::new(0, 1, 0));
    /// ```
    pub fn normal(&self, world_point: Point) -> Vector {
        // Transform world point to the object point
        let object_point = Point::from(self.transform_matrix.invert().unwrap() * world_point);

        let object_normal = object_point - Point::new(0, 0, 0);
        let world_normal =
            Vector::from(self.transform_matrix.invert().unwrap().transpose() * object_normal);

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts;

    #[test]
    fn new() {
        let s = Sphere::new();
        assert_eq!(s.transform_matrix, IDENTITY_MATRIX);
    }

    #[test]
    fn transform() {
        let s = Sphere::new();
        let t = Matrix::translate(2, 3, 4);
        let s = s.transform(t.clone());
        assert_eq!(s.transform_matrix, t);
    }

    #[test]
    fn normal() {
        // The normal on a sphere at a point on the x axis
        let s = Sphere::new();
        let n = s.normal(Point::new(1, 0, 0));
        assert_eq!(n, Vector::new(1, 0, 0));

        //The normal on a sphere at a point on the y axis
        let n = s.normal(Point::new(0, 1, 0));
        assert_eq!(n, Vector::new(0, 1, 0));

        //The normal on a sphere at a point on the z axis
        let n = s.normal(Point::new(0, 0, 1));
        assert_eq!(n, Vector::new(0, 0, 1));

        //The normal on a sphere at non axial point
        let n = s.normal(Point::new(
            (3. as f64).sqrt() / 3.,
            (3. as f64).sqrt() / 3.,
            (3. as f64).sqrt() / 3.,
        ));
        assert_eq!(
            n,
            Vector::new(
                (3. as f64).sqrt() / 3.,
                (3. as f64).sqrt() / 3.,
                (3. as f64).sqrt() / 3.,
            )
        );

        //The normal on a sphere at non axial point
        let n = s.normal(Point::new(
            (3. as f64).sqrt() / 3.,
            (3. as f64).sqrt() / 3.,
            (3. as f64).sqrt() / 3.,
        ));
        assert_eq!(
            n,
            Vector::new(
                (3. as f64).sqrt() / 3.,
                (3. as f64).sqrt() / 3.,
                (3. as f64).sqrt() / 3.,
            )
            .normalize()
        );

        // Computing a normal to a translated sphere
        let s = Sphere::new().transform(Matrix::translate(0, 1, 0));
        let n = s.normal(Point::new(0., 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0., 0.7071067811865475, -0.7071067811865476));

        // Computing a normal to a transformed sphere
        let s = Sphere::new();
        let m = Matrix::scale(1., 0.5, 1.) * Matrix::rotate_z(consts::PI / 5.);
        let s = s.transform(m);
        let n = s.normal(Point::new(
            0.,
            ((2. as f64).sqrt()) / 2.,
            (-(2. as f64).sqrt()) / 2.,
        ));
        assert_eq!(n, Vector::new(0., 0.9701425001453319, -0.24253562503633294));
    }
}
