//! Canvas and all it's functions
extern crate image;
use crate::units::color::QuantColor;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
/// Canvas is a datastructure that represents image canvas
#[derive(Debug)]
pub struct Canvas {
    /// Canvas width
    pub width: usize,
    /// Canvas height
    pub height: usize,
    /// Pixel storage of QuantColors
    pub pixels: Vec<Vec<QuantColor>>,
}

pub struct CanvasPart {
    pub size: usize,
    pub pixels: Vec<Vec<QuantColor>>,
}

impl Canvas {
    /// Returns new blank canvas of given width and height
    ///
    /// # Arguments
    ///
    /// * `width` - Canvas width
    /// * `height` - Canvas height
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::render::Canvas;
    ///
    /// let c = Canvas::new(10, 10);
    /// ```
    pub fn new(width: usize, height: usize) -> Canvas {
        let v = std::iter::repeat_with(|| {
            std::iter::repeat_with(|| QuantColor::new(0, 0, 0))
                .take(width)
                .collect()
        })
        .take(height as usize)
        .collect();
        Canvas {
            width,
            height,
            pixels: v,
        }
    }

    /// Writes a pixel at given coordinates
    ///
    /// If the coordinates are out of bounds it panics
    ///
    /// # Arguments
    ///
    /// * `x` - x coordinate
    /// * `y` - y coordinate
    /// * `color` - QuantColor to write
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::render::Canvas;
    /// use ::raytracer::units::color::QuantColor;
    /// let mut c = Canvas::new(10, 10);
    /// c.write_pixel(1, 1, QuantColor::new(255, 255, 30));
    /// ```
    ///
    pub fn write_pixel(&mut self, x: usize, y: usize, color: QuantColor) {
        if self.width <= x {
            panic!("x out of range, maximum width: {}", self.width);
        }
        if self.height <= y {
            panic!("y = {} out of range, maximum height: {}", y, self.height);
        }
        self.pixels[y][x] = color;
    }

    /// Returns a pixel at given coordinates
    ///
    /// If the coordinates are out of bounds it panics
    ///
    /// # Arguments
    /// * `x` - x coordinate
    /// * `y` - y coordinate
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::render::Canvas;
    /// let c = Canvas::new(10, 10);
    /// c.get_pixel(1, 1);
    /// ```
    pub fn get_pixel(&self, x: usize, y: usize) -> QuantColor {
        if self.width <= x {
            panic!("x out of range, maximum width: {}", self.width);
        }
        if self.height <= y {
            panic!("y out of range, maximum height: {}", self.height);
        }
        self.pixels[y][x]
    }

    /// Writes PPM file
    ///
    /// # Arguments
    ///
    /// * `path` - a location to write
    pub fn write_ppm(&self, path: &str) {
        let p = Path::new(path);
        let mut file = File::create(&p).unwrap();
        let headers: String = format!("P3\n{} {}\n255\n", self.height, self.width);
        file.write_all(headers.as_bytes()).unwrap();
        for row in self.pixels.iter() {
            let mut v: Vec<String> = Vec::new();
            for pixel in row.iter() {
                v.push((pixel.r).to_string());
                v.push((pixel.g).to_string());
                v.push((pixel.b).to_string());
            }
            for item in v.iter() {
                file.write_all(format!("{} ", item).as_bytes()).unwrap();
            }
            file.write_all("\n".as_bytes()).unwrap();
        }
    }

    /// Writes PNG file
    ///
    /// # Arguments
    ///
    /// * `path` - a location to write
    ///
    pub fn write_png(&self, path: &str) {
        let mut image: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        let path = Path::new(path);
        for column in 0..(self.width - 1) {
            for x in 0..(self.height - 1) {
                let r = self.pixels[x as usize][column as usize].r as u8;
                let g = self.pixels[x as usize][column as usize].g as u8;
                let b = self.pixels[x as usize][column as usize].b as u8;
                image.put_pixel(column as u32, x as u32, Rgb([r, g, b]));
            }
        }
        image.save_with_format(path, ImageFormat::Png).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(10, canvas.width);
        assert_eq!(20, canvas.height);
        assert_eq!(20, canvas.pixels.len());
        assert_eq!(10, canvas.pixels[0].len());
    }
    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let c = QuantColor::new(50, 50, 50);
        canvas.write_pixel(5, 5, c);
        assert_eq!(c, canvas.get_pixel(5, 5));
    }

    #[test]
    fn write_ppm() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel(3, 3, QuantColor::new(30, 50, 40));
        canvas.write_ppm("./target/image.ppm");
    }
    #[test]
    fn write_jpg() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel(3, 3, QuantColor::new(0, 130, 50));
        canvas.write_png("./target/image.jpg");
    }
}
