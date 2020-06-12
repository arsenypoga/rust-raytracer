use crate::units::color::{QuantColor, BLACK, WHITE};
use crate::units::objects::Shape;
use crate::units::tuple::Point;
use crate::units::{Matrix, Transformable, IDENTITY_MATRIX};
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PatternType {
    Stripe(QuantColor, QuantColor),
    Gradient(QuantColor, QuantColor),
    Ring(QuantColor, QuantColor),
    Checkers(QuantColor, QuantColor),
}

impl PatternType {
    pub fn color_at(&self, point: Point) -> QuantColor {
        match self {
            PatternType::Stripe(color_a, color_b) => self.stripe_color(point, *color_a, *color_b),
            PatternType::Gradient(color_a, color_b) => {
                self.gradient_color(point, *color_a, *color_b)
            }
            PatternType::Ring(color_a, color_b) => self.ring_color(point, *color_a, *color_b),
            PatternType::Checkers(color_a, color_b) => {
                self.checkers_color(point, *color_a, *color_b)
            }
        }
    }

    fn stripe_color(&self, point: Point, color_a: QuantColor, color_b: QuantColor) -> QuantColor {
        if point.x.floor() % 2. == 0. {
            color_a
        } else {
            color_b
        }
    }

    fn gradient_color(&self, point: Point, color_a: QuantColor, color_b: QuantColor) -> QuantColor {
        let distance: QuantColor = color_b - color_a;
        let fraction = point.x - point.x.floor();

        color_a + (distance * fraction)
    }

    fn ring_color(&self, point: Point, color_a: QuantColor, color_b: QuantColor) -> QuantColor {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2. == 0. {
            color_a
        } else {
            color_b
        }
    }

    fn checkers_color(&self, point: Point, color_a: QuantColor, color_b: QuantColor) -> QuantColor {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i64 % 2 == 0 {
            color_a
        } else {
            color_b
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pattern {
    pub transformation_matrix: Matrix,
    pub pattern_type: PatternType,
}

// impl ops::Index<usize> for Pattern {
//     type Output = QuantColor;

//     fn index(&self, index: usize) -> &QuantColor {}
// }

impl Pattern {
    pub fn new(pattern_type: PatternType) -> Pattern {
        Pattern {
            pattern_type,
            transformation_matrix: IDENTITY_MATRIX,
        }
    }

    // pub fn color_at(&self, point: Point) -> QuantColor {
    //     match self.pattern_type {
    //         PatternType::Stripe(color_a, color_b) => self.stripe_color(point),
    //         PatternType::Gradient(color_a, color_b) => self.gradient_color(point),
    //         PatternType::Ring(color_a, color_b) => self.ring_color(point),
    //         PatternType::Checkers(color_a, color_b) => self.checkers_color(point),
    //     }
    // }

    pub fn transform(&self, transformation_matrix: Matrix) -> Pattern {
        Pattern {
            transformation_matrix,
            ..*self
        }
    }

    pub fn set_pattern_type(&self, pattern_type: PatternType) -> Pattern {
        Pattern {
            pattern_type,
            ..*self
        }
    }

    pub fn color_at_object(&self, object: Shape, world_point: Point) -> QuantColor {
        let object_point =
            Point::from(object.transformation_matrix.invert().unwrap() * world_point);
        let pattern_point =
            Point::from(self.transformation_matrix.invert().unwrap() * object_point);
        self.pattern_type.color_at(pattern_point)
    }
}

impl Transformable for Pattern {
    fn transform(&self, transformation_matrix: Matrix) -> Self {
        Pattern {
            transformation_matrix,
            ..*self
        }
    }
    fn translate<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self {
        Pattern {
            transformation_matrix: self.transformation_matrix * Matrix::translate(x, y, z),
            ..*self
        }
    }
    fn scale<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self {
        Pattern {
            transformation_matrix: self.transformation_matrix * Matrix::scale(x, y, z),
            ..*self
        }
    }
    fn rotate_x<T: Into<f64> + Copy>(&self, r: T) -> Self {
        Pattern {
            transformation_matrix: self.transformation_matrix * Matrix::rotate_x(r),
            ..*self
        }
    }
    fn rotate_y<T: Into<f64> + Copy>(&self, r: T) -> Self {
        Pattern {
            transformation_matrix: self.transformation_matrix * Matrix::rotate_y(r),
            ..*self
        }
    }
    fn rotate_z<T: Into<f64> + Copy>(&self, r: T) -> Self {
        Pattern {
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
    ) -> Self {
        Pattern {
            transformation_matrix: self.transformation_matrix
                * Matrix::skew(x_to_y, x_to_z, y_to_x, y_to_z, z_to_x, z_to_y),
            ..*self
        }
    }
}

impl Default for Pattern {
    fn default() -> Pattern {
        Pattern {
            transformation_matrix: IDENTITY_MATRIX,
            pattern_type: PatternType::Stripe(WHITE, BLACK),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::objects::ObjectType;
    use crate::units::tuple::Tuple;
    #[test]
    fn new() {
        // let pattern = Pattern::new(PatternType::Stripe(WHITE, BLACK));
    }
    #[test]
    fn color_at() {
        // A stripe pattern is constant in y
        let p = Pattern::default();
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 1, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 2, 0)));

        //  A stripe pattern is constant in z
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 1)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 2)));

        // A stripe pattern alternates in x
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0.9, 0., 0.)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(1, 0, 0)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(-0.1, 0., 0.)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(-1, 0, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(-1.1, 0., 0.)));

        //  A gradient linearly interpolates between colors
        let p = Pattern::new(PatternType::Gradient(WHITE, BLACK));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(
            QuantColor::new(192, 192, 192),
            p.pattern_type.color_at(Point::new(0.25, 0., 0.))
        );
        assert_eq!(
            QuantColor::new(128, 128, 128),
            p.pattern_type.color_at(Point::new(0.5, 0., 0.))
        );
        assert_eq!(
            QuantColor::new(64, 64, 64),
            p.pattern_type.color_at(Point::new(0.75, 0., 0.))
        );

        // A ring should extend in both x and z
        let p = Pattern::new(PatternType::Ring(WHITE, BLACK));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(1, 0, 0)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(0, 0, 1)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(0.708, 0., 0.708)));

        // Checkers should repeat in x
        let p = Pattern::new(PatternType::Checkers(WHITE, BLACK));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0.99, 0., 0.)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(1.01, 0., 0.)));

        // Checkers should repeat in y
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0., 0.99, 0.)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(0., 1.01, 0.)));

        // Checkers should repeat in y
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0, 0, 0)));
        assert_eq!(WHITE, p.pattern_type.color_at(Point::new(0., 0., 0.99)));
        assert_eq!(BLACK, p.pattern_type.color_at(Point::new(0., 0., 1.01)));
    }

    #[test]
    fn color_at_object() {
        // Stripes with an object transformation
        let mut object = Shape::new(ObjectType::Sphere);
        object.transformation_matrix = Matrix::scale(2, 2, 2);
        let pattern = Pattern::new(PatternType::Stripe(WHITE, BLACK));
        assert_eq!(
            WHITE,
            pattern.color_at_object(object, Point::new(1.5, 0., 0.))
        );

        // Stripes with a pattern transformation
        let object = Shape::new(ObjectType::Sphere);
        let mut pattern = Pattern::new(PatternType::Stripe(WHITE, BLACK));
        pattern.transformation_matrix = Matrix::scale(2, 2, 2);
        assert_eq!(
            WHITE,
            pattern.color_at_object(object, Point::new(1.5, 0., 0.))
        );

        // Stripes with both an object and a pattern transformation
        let mut object = Shape::new(ObjectType::Sphere);
        object.transformation_matrix = Matrix::scale(2, 2, 2);
        let mut pattern = Pattern::new(PatternType::Stripe(WHITE, BLACK));
        pattern.transformation_matrix = Matrix::translate(0.5, 0., 0.);
        assert_eq!(
            WHITE,
            pattern.color_at_object(object, Point::new(2.5, 0., 0.))
        );
    }
}
