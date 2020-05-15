use crate::units::tuple::Tuple;
use crate::units::utils;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl Tuple for Vector {
    fn new<T: Into<f64>>(x: T, y: T, z: T) -> Vector {
        Vector {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 0.,
        }
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_z(&self) -> f64 {
        self.z
    }

    fn get_w(&self) -> f64 {
        self.w
    }
}

impl Vector {
    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, other: Vector) -> Vector {
        *self - other * 2. * self.dot(other)
    }
}

impl ops::Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, scalar: f64) -> Vector {
        Vector::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;
    fn div(self, scalar: f64) -> Vector {
        Vector::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        utils::float_eq(self.x, other.x)
            && utils::float_eq(self.y, other.y)
            && utils::float_eq(self.z, other.z)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let p = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 0.);
    }

    #[test]
    fn add() {
        let t1 = Vector::new(3, -2, 5);
        let t2 = Vector::new(-2, 3, 1);
        let t3 = t1 + t2;
        assert_eq!(t3, Vector::new(1, 1, 6));
    }

    #[test]
    fn substract() {
        let p1 = Vector::new(3.0, 2.0, 1.0);
        let p2 = Vector::new(5.0, 6.0, 7.0);
        let p3 = p1 - p2;
        assert_eq!(p3, Vector::new(-2.0, -4.0, -6.0));
    }
    #[test]
    fn negate() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let v2 = -v1;
        assert_eq!(v2, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn scale() {
        let t1 = Vector::new(1.0, -2.0, 3.0);
        let t2 = t1 * 3.5;
        assert_eq!(t2, Vector::new(3.5, -7.0, 10.5));

        let t2 = t1 * 0.5;
        assert_eq!(t2, Vector::new(0.5, -1.0, 1.5));
    }
    #[test]
    fn divide() {
        let t1 = Vector::new(1.0, -2.0, 3.0);
        let t2 = t1 / 2.0;
        assert_eq!(t2, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn magnitude() {
        let t1 = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(t1.magnitude(), 1.0);

        let t1 = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(t1.magnitude(), 1.0);

        let t1 = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(t1.magnitude(), 1.0);

        let t1 = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(t1.magnitude(), (14.0 as f64).sqrt());

        let t1 = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(t1.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalize() {
        let t1 = Vector::new(4.0, 0.0, 0.0);
        let t2 = t1.normalize();
        assert_eq!(t2, Vector::new(1.0, 0.0, 0.0));

        let t1 = Vector::new(1.0, 2.0, 3.0);
        let t2 = t1.normalize();
        assert_eq!(
            t2,
            Vector::new(
                1.0 / (14.0 as f64).sqrt(),
                2.0 / (14.0 as f64).sqrt(),
                3.0 / (14.0 as f64).sqrt()
            )
        );

        let t1 = Vector::new(1.0, 2.0, 3.0);
        let t2 = t1.normalize();
        assert_eq!(1.0, t2.magnitude());
    }

    #[test]
    fn dot() {
        let t1 = Vector::new(1.0, 2.0, 3.0);
        let t2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(t1.dot(t2), 20.0);
    }
    #[test]
    fn cross() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let expect = Vector::new(-1.0, 2.0, -1.0);

        assert_eq!(v1.cross(v2), expect);
        assert_eq!(v2.cross(v1), -expect);
    }
    #[test]
    fn reflect() {
        let v = Vector::new(1, -1, 0);
        let n = Vector::new(0, 1, 0);
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1, 1, 0));

        let v = Vector::new(0, -1, 0);
        let n = Vector::new((2.0 as f64).sqrt() / 2., (2.0 as f64).sqrt() / 2., 0.);
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1, 0, 0));
    }
}
