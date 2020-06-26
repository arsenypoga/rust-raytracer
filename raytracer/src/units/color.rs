//! Color manipulations
use std::ops;
/// Black QuantColor
pub const BLACK: QuantColor = QuantColor { r: 0, g: 0, b: 0 };
/// White QuantColor
pub const WHITE: QuantColor = QuantColor {
    r: 255,
    g: 255,
    b: 255,
};
/// Red QuantColor
pub const RED: QuantColor = QuantColor { r: 255, g: 0, b: 0 };
/// Green QuantColor
pub const GREEN: QuantColor = QuantColor { r: 0, g: 255, b: 0 };
/// Blue QuantColor
pub const BLUE: QuantColor = QuantColor { r: 0, g: 0, b: 255 };

/// QuantColor represents a color between 0 and 255
#[derive(Debug, Clone, Copy)]
pub struct QuantColor {
    /// Red
    pub r: i64,
    /// Green
    pub g: i64,
    /// Blue
    pub b: i64,
}

impl QuantColor {
    /// Returns new QuantColor
    ///
    /// # Arguments
    ///
    /// * `r` - red color
    /// * `g` - green color
    /// * `b` - blue color
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::color::QuantColor;
    /// let c1 = QuantColor::new(30, 30, 30);
    /// ```
    pub fn new(r: i64, g: i64, b: i64) -> QuantColor {
        QuantColor { r, g, b }
    }

    /// Returns clamped QuantColor
    ///
    /// if a color field is above 255 it sets it to 255
    /// if a color field is below 0 it sets it to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::color::QuantColor;
    /// let c1 = QuantColor::new(270, -15, 80);
    /// let c2 = c1.clamp();
    /// ```
    pub fn clamp(&self) -> QuantColor {
        let mut return_color = *self;
        if return_color.r > 255 {
            return_color.r = 255;
        } else if return_color.r < 0 {
            return_color.r = 0;
        }
        if return_color.g > 255 {
            return_color.g = 255;
        } else if return_color.g < 0 {
            return_color.g = 0;
        }
        if return_color.b > 255 {
            return_color.b = 255;
        } else if return_color.b < 0 {
            return_color.b = 0;
        }
        return_color
    }
}

impl ops::Add for QuantColor {
    type Output = QuantColor;
    fn add(self, other: QuantColor) -> QuantColor {
        QuantColor::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl ops::Sub for QuantColor {
    type Output = QuantColor;
    fn sub(self, other: QuantColor) -> QuantColor {
        QuantColor::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl ops::Mul for QuantColor {
    type Output = QuantColor;
    fn mul(self, other: QuantColor) -> QuantColor {
        QuantColor::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl PartialEq for QuantColor {
    fn eq(&self, other: &QuantColor) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl ops::Mul<f64> for QuantColor {
    type Output = QuantColor;
    fn mul(self, scalar: f64) -> QuantColor {
        QuantColor::new(
            (self.r as f64 * scalar) as i64,
            (self.g as f64 * scalar) as i64,
            (self.b as f64 * scalar) as i64,
        )
    }
}

impl ops::Mul<QuantColor> for f64 {
    type Output = QuantColor;
    fn mul(self, other: QuantColor) -> QuantColor {
        QuantColor::new(
            (self * other.r as f64) as i64,
            (self * other.g as f64) as i64,
            (self * other.b as f64) as i64,
        )
    }
}

impl ops::Mul<i64> for QuantColor {
    type Output = QuantColor;
    fn mul(self, scalar: i64) -> Self {
        QuantColor::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

impl ops::Mul<QuantColor> for i64 {
    type Output = QuantColor;
    fn mul(self, other: QuantColor) -> QuantColor {
        QuantColor::new(self * other.r, self * other.g, self * other.b)
    }
}

impl Default for QuantColor {
    fn default() -> Self {
        WHITE
    }
}

impl From<[i64; 3]> for QuantColor {
    fn from(a: [i64; 3]) -> Self {
        QuantColor::new(a[0], a[1], a[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let c = QuantColor::new(7, 5, 8);
        assert_eq!(7, c.r);
        assert_eq!(5, c.g);
        assert_eq!(8, c.b);
    }

    #[test]
    fn add() {
        let c1 = QuantColor::new(9, 6, 75);
        let c2 = QuantColor::new(7, 1, 25);
        let expect = QuantColor::new(16, 7, 100);
        assert_eq!(expect, c1 + c2);
    }
    #[test]
    fn substract() {
        let c1 = QuantColor::new(9, 6, 75);
        let c2 = QuantColor::new(7, 1, 25);
        let expect = QuantColor::new(2, 5, 50);
        assert_eq!(expect, c1 - c2);
    }
    #[test]
    fn scale() {
        let c1 = QuantColor::new(2, 3, 4);
        let expect = QuantColor::new(4, 6, 8);
        assert_eq!(expect, c1 * 2);
    }
    #[test]
    fn multiply() {
        let c1 = QuantColor::new(1, 2, 40);
        let c2 = QuantColor::new(9, 1, 1);
        let expect = QuantColor::new(9, 2, 40);
        assert_eq!(expect, c1 * c2);
    }
}
