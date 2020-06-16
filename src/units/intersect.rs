//! Intersection operations
use crate::units::objects::Shape;
use crate::units::tuple::{Point, Vector};
use crate::units::utils;
use crate::units::Ray;
use std::cmp::Ordering;
/// Intersection is a struct representing a point of intersection.
#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    /// Point of intersection
    pub t: f64,
    /// Intersected object
    pub object: &'a Shape,
}

/// Computations is a struct that stores computations that are used for various computations :3
#[derive(Debug, Clone)]
pub struct Computations<'a> {
    /// T of intersection for the ray
    pub t: f64,
    /// Object of computations
    pub object: &'a Shape,
    /// Point of intersections
    pub point: Point,
    /// Point for approximation
    pub over_point: Point,
    pub under_point: Point,
    /// Eye vector
    pub eyev: Vector,
    /// Normal vector at the point
    pub normalv: Vector,
    /// Reflection vector at the point
    pub reflectv: Vector,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Intersection<'a> {
    /// Creates new Intersection
    pub fn new(t: f64, object: &'a Shape) -> Intersection<'a> {
        Intersection { t, object }
    }

    /// Returns Intersection that is hit from given set of Intersections
    ///
    /// The return value is wrapped in an Option.
    /// If there are no intersections, the return value is None, Otherwise the Some(Intersection) is returned
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
    /// Returns base computations, that is computations with n1 and n2 set to 1.
    pub fn base_computations(&self, ray: Ray) -> Computations {
        let position = ray.position(self.t);
        let mut normalv = self.object.normal(position);
        let eyev = -ray.direction;
        let inside = normalv.dot(eyev) < 0.;

        if inside {
            normalv = -normalv;
        }

        Computations {
            t: self.t,
            object: self.object,
            point: position,
            eyev: -ray.direction,
            normalv,
            inside,
            over_point: position + normalv * utils::EPSILON,
            under_point: Point::from(position - Point::from(normalv * utils::EPSILON)),
            reflectv: ray.direction.reflect(normalv),
            n1: 1.,
            n2: 1.,
        }
    }
    /// Returns computations with the computed n1 and n2.
    ///
    /// # Arguments
    /// * `r` - A ray at which the Computations are computed.
    /// * `intersections` - Pointer to a vector with intersections
    ///
    /// # Returns
    /// Populated Computations
    pub fn computations(&self, r: Ray, intersections: &Vec<Intersection>) -> Computations {
        let mut comps = self.base_computations(r);
        let (n1, n2) = self.compute_refraction_indexes(intersections);
        comps.n1 = n1;
        comps.n2 = n2;
        comps
    }

    /// Computes refraction indexes for the given computation at the intersections
    ///
    /// # Arguments
    /// * `intersections` - Pointer to a vector with intersections
    ///
    /// # Returns
    /// (n1, n2)
    fn compute_refraction_indexes(&self, intersections: &Vec<Intersection>) -> (f64, f64) {
        let mut containers: Vec<&Shape> = Vec::new();
        let mut n1 = 1.;
        let mut n2 = 1.;

        for intersection in intersections {
            if self.t == intersection.t {
                if !containers.is_empty() {
                    n1 = containers.last().unwrap().material.refractive_index;
                }
            }

            if containers.contains(&intersection.object) {
                let i = containers
                    .iter()
                    .position(|&item| item == intersection.object)
                    .unwrap();
                containers.remove(i);
            } else {
                containers.push(intersection.object);
            }

            if self.t == intersection.t {
                if !containers.is_empty() {
                    n2 = containers.last().unwrap().material.refractive_index;
                }
                break;
            }
        }
        (n1, n2)
    }
}
impl Computations<'_> {
    /// Computes how much schlick refraction is applied
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot(self.normalv);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();

            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
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
    use crate::units::utils;
    use crate::units::{Matrix, Transformable};
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
        let comps = i.base_computations(r);
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
        let comps = i.base_computations(r);
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
        let comps = i.base_computations(r);
        assert!(comps.over_point.z < -utils::EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);

        // Precomputing the reflection vector
        let s = Shape::new(ObjectType::Plane);
        let r = Ray::new(
            Point::new(0, 1, -1),
            Vector::new(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new(2_f64.sqrt(), &s);
        let comps = i.base_computations(r);
        assert_eq!(
            comps.reflectv,
            Vector::new(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
        );

        // Finding n1 and n2 at various intersections
        let mut a = Shape::glass_sphere().scale(2, 2, 2);
        a.material.refractive_index = 1.5;

        let mut b = Shape::glass_sphere().translate(0., 0., -0.25);
        b.material.refractive_index = 2.;

        let mut c = Shape::glass_sphere().translate(0., 0., 0.25);
        c.material.refractive_index = 2.5;

        let r = Ray::new(Point::new(0, 0, -4), Vector::new(0, 0, 1));
        let ints = vec![
            Intersection::new(2., &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6., &a),
        ];

        let comps = ints[0].computations(r, &ints);
        assert_eq!(1., comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = ints[1].computations(r, &ints);
        assert_eq!(1.5, comps.n1);
        assert_eq!(2., comps.n2);

        let comps = ints[2].computations(r, &ints);
        assert_eq!(2., comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = ints[3].computations(r, &ints);
        assert_eq!(2.5, comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = ints[4].computations(r, &ints);
        assert_eq!(2.5, comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = ints[5].computations(r, &ints);
        assert_eq!(1.5, comps.n1);
        assert_eq!(1., comps.n2);

        // The under point is offset below the surface
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Shape::glass_sphere().translate(0, 0, 1);
        let i = Intersection::new(5., &shape);
        let comps = i.base_computations(r);
        assert!(comps.under_point.z > utils::EPSILON / 2.);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick() {
        // The Schlick approximation under total internal reflection
        let shape = Shape::glass_sphere();
        let r = Ray::new(Point::new(0., 0., 2_f64.sqrt() / 2.), Vector::new(0, 1, 0));
        let ints = vec![
            Intersection::new(-(2_f64.sqrt() / 2.), &shape),
            Intersection::new(2_f64.sqrt() / 2., &shape),
        ];

        let comps = ints[1].computations(r, &ints);
        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.);

        // The Schlick approximation with a perpendicular viewing angle
        let shape = Shape::glass_sphere();
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0, 1, 0));
        let ints = vec![
            Intersection::new(-1., &shape),
            Intersection::new(1., &shape),
        ];

        let comps = ints[1].computations(r, &ints);
        let reflectance = comps.schlick();
        assert!(utils::float_eq(reflectance, 0.04));

        // The Schlick approximation with small angle and n2 > n1
        let shape = Shape::glass_sphere();
        let r = Ray::new(Point::new(0., 0.99, -2.), Vector::new(0, 0, 1));
        let ints = vec![Intersection::new(1.8589, &shape)];

        let comps = ints[0].computations(r, &ints);
        let reflectance = comps.schlick();
        assert!(utils::float_eq(reflectance, 0.48873));
    }
}
