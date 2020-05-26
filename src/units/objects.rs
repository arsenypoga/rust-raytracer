pub use crate::units::tuple::{Point, Tuple, Vector, ORIGIN};
pub use crate::units::utils;
use crate::units::{Intersection, Matrix, Ray, IDENTITY_MATRIX};
pub use crate::world::Material;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectType {
    Sphere,
    Plane,
}

impl ObjectType {}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Shape {
    pub transformation_matrix: Matrix,
    pub material: Material,
    pub object_type: ObjectType,
}

impl Shape {
    pub fn new(object_type: ObjectType) -> Shape {
        Shape {
            transformation_matrix: IDENTITY_MATRIX,
            material: Material::default(),
            object_type,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transformation_matrix.invert().unwrap());
        match self.object_type {
            ObjectType::Sphere => self.intersect_sphere(local_ray),
            ObjectType::Plane => self.intersect_plane(local_ray),
        }
    }

    fn intersect_sphere(&self, local_ray: Ray) -> Vec<Intersection> {
        let Ray { origin, direction } = local_ray;

        let distance = origin - Point::new(0, 0, 0);

        let a = direction.dot(direction);
        let b = 2. * direction.dot(distance);
        let c = distance.dot(distance) - 1.;

        let discriminant = b.powi(2) - (4. * a * c);
        if discriminant < 0. {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);
        vec![
            Intersection {
                t: t1,
                object: self,
            },
            Intersection {
                t: t2,
                object: self,
            },
        ]
    }

    fn intersect_plane(&self, local_ray: Ray) -> Vec<Intersection> {
        if local_ray.direction.y.abs() < utils::EPSILON {
            vec![]
        } else {
            vec![Intersection::new(
                -(local_ray.origin.y / local_ray.direction.y),
                self,
            )]
        }
    }

    pub fn normal(&self, point: Point) -> Vector {
        let local_point = Point::from(self.transformation_matrix.invert().unwrap() * point);
        let local_normal = match self.object_type {
            ObjectType::Sphere => local_point - ORIGIN,
            ObjectType::Plane => Vector::new(0, 1, 0),
        };

        Vector::from(self.transformation_matrix.invert().unwrap().transpose() * local_normal)
            .normalize()
    }

    pub fn transform(&self, matrix: Matrix) -> Shape {
        Shape {
            transformation_matrix: matrix,
            ..Shape::default()
        }
    }
}

impl Default for Shape {
    fn default() -> Shape {
        Shape {
            transformation_matrix: IDENTITY_MATRIX,
            material: Material::default(),
            object_type: ObjectType::Sphere,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts;
    #[test]
    fn new_sphere() {
        // Test Sphere
        let mut s = Shape::new(ObjectType::Sphere);
        assert_eq!(s.transformation_matrix, IDENTITY_MATRIX);

        s.transformation_matrix = Matrix::translate(2, 3, 4);
        assert_eq!(s.transformation_matrix, Matrix::translate(2, 3, 4));

        assert_eq!(s.material, Material::default());
    }
    // TODO: new_plane()

    #[test]
    fn intersect() {
        // Ray intersects sphere at two points.
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Shape::new(ObjectType::Sphere);
        let i = s.intersect(r);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, 4.0);
        assert_eq!(i[1].t, 6.0);

        // Ray intersects sphere at a tangent.
        let r = Ray::new(Point::new(0, 1, -5), Vector::new(0, 0, 1));
        let s = Shape::new(ObjectType::Sphere);

        let i = s.intersect(r);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, 5.0);
        assert_eq!(i[1].t, 5.0);

        // Ray misses the point
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let s = Shape::new(ObjectType::Sphere);

        let i = s.intersect(r);
        assert_eq!(i.len(), 0);

        // Ray originates inside of the sphere
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = Shape::new(ObjectType::Sphere);

        let i = s.intersect(r);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, -1.0);
        assert_eq!(i[1].t, 1.0);

        // Sphere is behind a ray
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let s = Shape::new(ObjectType::Sphere);
        let i = s.intersect(r);
        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, -6.0);
        assert_eq!(i[1].t, -4.0);

        // Intersecting a scaled sphere with a ray
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Shape::new(ObjectType::Sphere);

        s.transformation_matrix = Matrix::scale(2, 2, 2);
        let i = s.intersect(r);

        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, 3.0);
        assert_eq!(i[1].t, 7.0);

        // Intersecting a translated sphere with a ray
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Shape::new(ObjectType::Sphere);
        s.transformation_matrix = Matrix::translate(5, 0, 0);
        let i = s.intersect(r);

        assert_eq!(i.len(), 0);
    }

    #[test]
    fn plane_intersect() {
        // Intersect with a ray parallel to the plane
        let p = Shape::new(ObjectType::Plane);
        let r = Ray::new(Point::new(0, 10, 0), Vector::new(0, 0, 1));
        let xs = p.intersect(r);
        assert!(xs.is_empty());

        // Intersect with a coplanar ray
        let r = Ray::new(ORIGIN, Vector::new(0, 0, 1));
        let xs = p.intersect(r);
        assert!(xs.is_empty());

        // A ray intersecting a plane from above
        let r = Ray::new(Point::new(0, 1, 0), Vector::new(0, -1, 0));
        let xs = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, &p);

        // A ray intersecting a plane from below
        let r = Ray::new(Point::new(0, -1, 0), Vector::new(0, 1, 0));
        let xs = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, &p);
    }

    #[test]
    fn sphere_normal() {
        // The normal on a sphere at a point on the x axis
        let s = Shape::new(ObjectType::Sphere);
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
        let s = Shape::new(ObjectType::Sphere).transform(Matrix::translate(0, 1, 0));
        let n = s.normal(Point::new(0., 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0., 0.7071067811865475, -0.7071067811865476));

        // Computing a normal to a transformed sphere
        let s = Shape::new(ObjectType::Sphere);
        let m = Matrix::scale(1., 0.5, 1.) * Matrix::rotate_z(consts::PI / 5.);
        let s = s.transform(m);
        let n = s.normal(Point::new(
            0.,
            ((2. as f64).sqrt()) / 2.,
            (-(2. as f64).sqrt()) / 2.,
        ));
        assert_eq!(n, Vector::new(0., 0.9701425001453319, -0.24253562503633294));
    }

    #[test]
    fn plane_normal() {
        let p = Shape::new(ObjectType::Plane);
        assert_eq!(p.normal(Point::new(0, 0, 0)), Vector::new(0, 1, 0));
        assert_eq!(p.normal(Point::new(10, 0, -10)), Vector::new(0, 1, 0));
        assert_eq!(p.normal(Point::new(-5, 0, 150)), Vector::new(0, 1, 0));
    }
}
