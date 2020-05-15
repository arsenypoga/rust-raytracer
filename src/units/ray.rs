//! Ray operations
use crate::units::tuple::{Point, Vector};
use crate::units::{Intersection, Matrix, Sphere};
/// Ray is a simply line.
///
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    /// Starting point of a Ray.
    pub origin: Point,
    /// Direction of a Ray.
    pub direction: Vector,
}

impl Ray {
    /// Creates a new Ray.
    ///
    /// # Arguments
    ///
    /// * `origin` - a tuple, must be a point.
    /// * `direction` - a Tuple, must be a vector.
    ///
    /// If these conditions aren't met, a panic will occur.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::Ray;
    /// use ::raytracer::units::tuple::{Point, Vector, Tuple};
    ///
    /// let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 1, 0));
    /// ```
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    /// Returns a Tuple point from given travel time
    ///
    /// # Arguments
    ///
    /// `t` - ray travel time
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::Ray;
    /// use ::raytracer::units::tuple::{Tuple, Point, Vector};
    ///
    /// let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 1, 0));
    /// r.position(7);
    /// ```
    pub fn position<T: Into<f64>>(&self, t: T) -> Point {
        self.origin + self.direction * t.into()
    }

    /// Returns Intersection of a Sphere with a Ray
    ///
    /// # Arguments
    /// * `ray` - a Ray struct that is cast onto the Sphere
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::{Sphere, Ray};
    /// use ::raytracer::units::tuple::{Point, Tuple, Vector};
    /// let s = Sphere::new();
    /// let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 1, 0));
    ///
    ///
    ///
    /// ```
    pub fn intersect<'a>(&self, object: &'a Sphere) -> Vec<Intersection<'a>> {
        let r2 = self.transform(object.transform_matrix.invert().unwrap());

        let distance = r2.origin - object.origin;

        let a = r2.direction.dot(r2.direction);
        let b = 2. * r2.direction.dot(distance);
        let c = distance.dot(distance) - 1.;

        let discriminant = b.powi(2) - (4. * a * c);
        if discriminant < 0. {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);
        vec![
            Intersection { t: t1, object },
            Intersection { t: t2, object },
        ]
    }

    /// Translates
    pub fn transform(self, m: Matrix) -> Ray {
        Ray {
            origin: Point::from(m * self.origin),
            direction: Vector::from(m * self.direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::tuple::Tuple;

    #[test]
    fn new_ray() {
        let o = Point::new(1, 2, 3);
        let d = Vector::new(4, 5, 6);
        let r = Ray::new(o, d);
        assert_eq!(r.origin, o);
        assert_eq!(r.direction, d);
    }

    #[test]
    fn position() {
        let o = Point::new(2, 3, 4);
        let d = Vector::new(1, 0, 0);
        let r = Ray::new(o, d);

        assert_eq!(Point::new(2, 3, 4), r.position(0));
        assert_eq!(Point::new(3, 3, 4), r.position(1));
        assert_eq!(Point::new(1, 3, 4), r.position(-1));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.position(2.5));
    }
    #[test]
    fn intersect() {
        // Ray intersects sphere at two points.
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, 4.0);
        assert_eq!(i[1].t, 6.0);

        // Ray intersects sphere at a tangent.
        let r = Ray::new(Point::new(0, 1, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, 5.0);
        assert_eq!(i[1].t, 5.0);

        // Ray misses the point
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.len(), 0);

        // Ray originates inside of the sphere
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, -1.0);
        assert_eq!(i[1].t, 1.0);

        // Sphere is behind a ray
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, -6.0);
        assert_eq!(i[1].t, -4.0);

        // Intersecting a scaled sphere with a ray
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let s = s.transform(Matrix::scale(2, 2, 2));
        let i = r.intersect(&s);

        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, 3.0);
        assert_eq!(i[1].t, 7.0);

        // Intersecting a translated sphere with a ray
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let s = s.transform(Matrix::translate(5, 0, 0));
        let i = r.intersect(&s);

        assert_eq!(i.len(), 0);
    }

    #[test]
    fn transform() {
        // Translating a ray
        let r1 = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Matrix::translate(3, 4, 5);
        let r2 = r1.transform(m);
        println!("{:?}", r2);

        assert_eq!(r2.origin, Point::new(4, 6, 8));
        assert_eq!(r2.direction, Vector::new(0, 1, 0));

        // Scaling a ray

        let r1 = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Matrix::scale(2, 3, 4);
        let r2 = r1.transform(m);

        assert_eq!(r2.origin, Point::new(2, 6, 12));
        assert_eq!(r2.direction, Vector::new(0, 3, 0));
    }
}
