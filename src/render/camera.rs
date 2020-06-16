use crate::render::{Canvas, World};
use crate::units::tuple::{Point, Tuple};
use crate::units::Ray;
use crate::units::{Matrix, Transformable, IDENTITY_MATRIX};
use rayon::prelude::*;
use std::sync::Mutex;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transformation_matrix: Matrix,
    pub pixel_size: f64,
    pub half_height: f64,
    pub half_width: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.).tan();

        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        Camera {
            hsize,
            vsize,
            field_of_view,
            transformation_matrix: IDENTITY_MATRIX,
            pixel_size: (half_width * 2.) / (hsize as f64),
            half_height,
            half_width,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width as f64 - xoffset;
        let world_y = self.half_height as f64 - yoffset;

        let invert_transform = self.transformation_matrix.invert().unwrap();
        let pixel = Point::from(invert_transform * Point::new(world_x, world_y, -1.));
        let origin = Point::from(invert_transform * Point::new(0, 0, 0));
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let canvas = Mutex::new(Canvas::new(self.hsize, self.vsize));
        (0..self.hsize).into_par_iter().for_each(|y| {
            (0..self.vsize).into_par_iter().for_each(|x| {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, 5);
                let mut canvas = canvas.lock().unwrap();
                canvas.write_pixel(x, y, color);
            })
        });
        canvas.into_inner().unwrap()
    }
}

impl Transformable for Camera {
    fn translate<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self {
        Camera {
            transformation_matrix: self.transformation_matrix * Matrix::translate(x, y, z),
            ..*self
        }
    }
    fn scale<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self {
        Camera {
            transformation_matrix: self.transformation_matrix * Matrix::scale(x, y, z),
            ..*self
        }
    }
    fn rotate_x<T: Into<f64> + Copy>(&self, r: T) -> Self {
        Camera {
            transformation_matrix: self.transformation_matrix * Matrix::rotate_x(r),
            ..*self
        }
    }
    fn rotate_y<T: Into<f64> + Copy>(&self, r: T) -> Self {
        Camera {
            transformation_matrix: self.transformation_matrix * Matrix::rotate_y(r),
            ..*self
        }
    }
    fn rotate_z<T: Into<f64> + Copy>(&self, r: T) -> Self {
        Camera {
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
        Camera {
            transformation_matrix: self.transformation_matrix
                * Matrix::skew(x_to_y, x_to_z, y_to_x, y_to_z, z_to_x, z_to_y),
            ..*self
        }
    }
    fn transform(&self, transformation_matrix: Matrix) -> Self {
        Camera {
            transformation_matrix,
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::color::QuantColor;
    use crate::units::tuple::{Tuple, Vector};
    use crate::units::utils;
    use std::f64::consts;
    #[test]
    fn new() {
        let c = Camera::new(160, 120, consts::FRAC_PI_2);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, consts::FRAC_PI_2);
        assert_eq!(c.transformation_matrix, IDENTITY_MATRIX);

        // Pixel on a horizontal canvas
        let c = Camera::new(200, 125, consts::FRAC_PI_2);
        println!("{:?}", c.pixel_size);

        assert!(utils::float_eq(c.pixel_size, 0.01));

        // Pixel size on vertical canvas
        let c = Camera::new(125, 200, consts::FRAC_PI_2);
        println!("{:?}", c.pixel_size);
        assert!(utils::float_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn ray_for_pixel() {
        // Constructing a ray through the center of the canvas
        let c = Camera::new(201, 101, consts::FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0, 0, 0));
        assert_eq!(r.direction, Vector::new(0, 0, -1));

        // Constructing a ray through a corner of the canvas
        let c = Camera::new(201, 101, consts::FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Point::new(0, 0, 0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));

        // Constructing a ray when the camera is transformed
        let c = Camera::new(201, 101, consts::FRAC_PI_2)
            .rotate_y(consts::FRAC_PI_4)
            .translate(0, -2, 5);
        // .transform(Matrix::rotate_y(consts::FRAC_PI_4) * Matrix::translate(0, -2, 5));
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0, 2, -5));
        assert_eq!(
            r.direction,
            Vector::new(2_f64.sqrt() / 2., 0., -2_f64.sqrt() / 2.)
        );
    }

    #[test]
    fn render() {
        let w = World::default();
        let c = Camera::new(11, 11, consts::FRAC_PI_2).transform(Matrix::view_transform(
            Point::new(0, 0, -5),
            Point::new(0, 0, 0),
            Vector::new(0, 1, 0),
        ));
        let image = c.render(w);

        assert_eq!(image.get_pixel(5, 5), QuantColor::new(96, 120, 72));
    }
}
