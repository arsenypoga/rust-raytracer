use crate::units::color::{QuantColor, BLACK};
use crate::units::tuple::{Point, Tuple, Vector, ORIGIN};
use crate::units::utils;
use crate::units::{Intersection, Matrix, Ray, Transformable, IDENTITY_MATRIX};
use crate::world::{Material, PointLight};
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectType {
    Sphere,
    Plane,
}

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

    pub fn set_material(&self, material: Material) -> Shape {
        Shape { material, ..*self }
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

    pub fn lightning(
        &self,
        light: PointLight,
        position: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> QuantColor {
        let intensity = QuantColor::new(
            light.intensity.r / 255,
            light.intensity.g / 255,
            light.intensity.b / 255,
        );
        let color = if self.material.pattern.is_some() {
            self.material
                .pattern
                .unwrap()
                .color_at_object(*self, position)
        } else {
            self.material.color
        };

        let effective_color = (color * intensity).clamp();
        let lightv = (light.position - position).normalize();

        let ambient = (effective_color * self.material.ambient as f64).clamp();
        let diffuse;
        let specular;

        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0. {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = (effective_color * self.material.diffuse as f64 * light_dot_normal).clamp();
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0. {
                specular = BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.material.shine);
                specular = (light.intensity * self.material.specular as f64 * factor).clamp();
            }
        }
        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }
}

impl Transformable for Shape {
    fn translate<T: Into<f64>>(&self, x: T, y: T, z: T) -> Shape {
        Shape {
            transformation_matrix: self.transformation_matrix * Matrix::translate(x, y, z),
            ..*self
        }
    }
    fn scale<T: Into<f64>>(&self, x: T, y: T, z: T) -> Shape {
        Shape {
            transformation_matrix: self.transformation_matrix * Matrix::scale(x, y, z),
            ..*self
        }
    }
    fn rotate_x<T: Into<f64> + Copy>(&self, r: T) -> Shape {
        Shape {
            transformation_matrix: self.transformation_matrix * Matrix::rotate_x(r),
            ..*self
        }
    }
    fn rotate_y<T: Into<f64> + Copy>(&self, r: T) -> Shape {
        Shape {
            transformation_matrix: self.transformation_matrix * Matrix::rotate_y(r),
            ..*self
        }
    }
    fn rotate_z<T: Into<f64> + Copy>(&self, r: T) -> Shape {
        Shape {
            transformation_matrix: self.transformation_matrix * Matrix::rotate_z(r),
            ..*self
        }
    }
    fn skew<T: Into<f64> + Copy>(
        &self,
        x_to_y: T,
        x_to_z: T,
        y_to_x: T,
        y_to_z: T,
        z_to_x: T,
        z_to_y: T,
    ) -> Shape {
        Shape {
            transformation_matrix: self.transformation_matrix
                * Matrix::skew(x_to_y, x_to_z, y_to_x, y_to_z, z_to_x, z_to_y),
            ..*self
        }
    }
    fn transform(&self, transformation_matrix: Matrix) -> Self {
        Shape {
            transformation_matrix,
            ..*self
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
    use crate::units::color::WHITE;
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

    #[test]
    fn lightning() {
        let o = Shape::default();
        let p = Point::new(0, 0, 0);

        // Lighting with the eye between the light and the surface
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), QuantColor::new(255, 255, 255));
        let res = o.lightning(light, p, eyev, normalv, false);
        assert_eq!(res, QuantColor::new(483, 483, 483));

        // Lighting with the eye between light and surface, eye offset 45°
        let eyev = Vector::new(0., (2.0 as f64).sqrt() / 2., (2.0 as f64).sqrt() / 2.);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), WHITE);
        let res = o.lightning(light, p, eyev, normalv, false).clamp();
        assert_eq!(res, QuantColor::new(254, 254, 254));

        // Lighting with eye opposite surface, light offset 45°
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), QuantColor::new(255, 255, 255));
        let res = o.lightning(light, p, eyev, normalv, false);
        assert_eq!(res, QuantColor::new(186, 186, 186));

        // Lighting with eye in the path of the reflection vector
        let eyev = Vector::new(0., -(2.0 as f64).sqrt() / 2., -(2.0 as f64).sqrt() / 2.);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), QuantColor::new(255, 255, 255));
        let res = o.lightning(light, p, eyev, normalv, false);
        assert_eq!(res, QuantColor::new(415, 415, 415));

        // Lighting with the light behind the surface
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, 10), QuantColor::new(255, 255, 255));
        let res = o.lightning(light, p, eyev, normalv, false);
        assert_eq!(res, QuantColor::new(25, 25, 25));

        // Lighting with the surface in shadow
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, 10), QuantColor::new(255, 255, 255));
        let res = o.lightning(light, p, eyev, normalv, true);
        assert_eq!(res, QuantColor::new(25, 25, 25));
    }
}
