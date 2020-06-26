use crate::units::tuple::{Tuple, Vector};
use crate::units::utils;
use crate::units::Matrix;
use std::ops;

pub const ORIGIN: Point = Point {
    x: 0.,
    y: 0.,
    z: 0.,
    w: 1.,
};

/// Point is a Tuple with four fields.
/// the last field is not public, because it's what differenciates Point from Tuple.
/// In the case of Point the w is 1
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl Tuple for Point {
    fn new<T: Into<f64>>(x: T, y: T, z: T) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1.,
        }
    }
    /// Accesses x value
    fn get_x(&self) -> f64 {
        self.x
    }
    /// Accesses y value
    fn get_y(&self) -> f64 {
        self.y
    }

    /// Accesses z value
    fn get_z(&self) -> f64 {
        self.z
    }

    /// Accesses w value
    fn get_w(&self) -> f64 {
        self.w
    }
}

impl ops::Add for Point {
    type Output = Vector;
    fn add(self, other: Point) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, scalar: f64) -> Point {
        Point::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, scalar: f64) -> Point {
        Point::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        utils::float_eq(self.x, other.x)
            && utils::float_eq(self.y, other.y)
            && utils::float_eq(self.z, other.z)
    }
}

impl From<Matrix> for Point {
    fn from(m: Matrix) -> Point {
        Point::new(m[0][0], m[1][0], m[2][0])
    }
}

impl From<Vector> for Point {
    fn from(v: Vector) -> Self {
        Point::new(v.x, v.y, v.z)
    }
}

impl From<[f64; 3]> for Point {
    fn from(a: [f64; 3]) -> Self {
        Point::new(a[0], a[1], a[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.);
    }

    #[test]
    fn add() {
        let t1 = Point::new(3, -2, 5);
        let t2 = Point::new(-2, 3, 1);
        let t3 = t1 + t2;
        assert_eq!(t3, Vector::new(1, 1, 6));
    }

    #[test]
    fn substract() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let p3 = p1 - p2;
        assert_eq!(p3, Vector::new(-2.0, -4.0, -6.0));
    }
    #[test]
    fn negate() {
        let v1 = Point::new(1.0, -2.0, 3.0);
        let v2 = -v1;
        assert_eq!(v2, Point::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn scale() {
        let t1 = Point::new(1.0, -2.0, 3.0);
        let t2 = t1 * 3.5;
        assert_eq!(t2, Point::new(3.5, -7.0, 10.5));

        let t2 = t1 * 0.5;
        assert_eq!(t2, Point::new(0.5, -1.0, 1.5));
    }
    #[test]
    fn divide() {
        let t1 = Point::new(1.0, -2.0, 3.0);
        let t2 = t1 / 2.0;
        assert_eq!(t2, Point::new(0.5, -1.0, 1.5));
    }
}
