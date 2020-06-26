//! Basic matrix operations

use crate::units::tuple::{Point, Tuple, Vector};
use std::ops;
/// Represents a two dimensional Matrix
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    /// Column count
    pub width: usize,
    /// Row count
    pub height: usize,
    data: [[f64; 4]; 4],
}

pub const IDENTITY_MATRIX: Matrix = Matrix {
    width: 4,
    height: 4,
    data: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Matrix {
    /// Returns new matrix of given width and height, and fills it with zeroes.
    ///
    /// # Arguments
    ///
    /// * `width` - width of matrix
    /// * `height` - heigtht of matrix
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::Matrix;
    ///
    /// let m = Matrix::new(10, 10);
    /// ```
    ///
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            data: [
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
            ],
        }
    }

    /// Returns a transposed Matrix. (Matrix with x and y switched)
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::Matrix;
    /// let m1 = Matrix::from([
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [9.0, 10.0, 11.0, 12.0],
    ///     [13.0, 14.0, 15.0, 16.0],
    /// ]);
    ///
    /// let m2 = m1.transpose();
    /// ```
    pub fn transpose(&self) -> Matrix {
        let height = self.height;
        let width = self.width;
        let mut return_matrix = Matrix::new(width, height);
        for i in 0..height {
            for j in 0..width {
                return_matrix[j][i] = self.data[i][j];
            }
        }
        return_matrix
    }

    /// Returns submatrix with row and col removed
    ///
    /// # Arguments
    ///
    /// * `row` - a row to remove
    /// * `col` - a column to remove
    ///
    /// #Examples
    ///
    /// ```
    /// use ::raytracer::units::Matrix;
    ///let a = Matrix::from([
    ///         [1.0, 5.0, 0.0, 0.],
    ///         [-3.0, 2.0, 7.0, 0.],
    ///         [0.0, 6.0, -3.0, 0.],
    ///         [0.0, 0.0, 0.0, 0.],
    ///     ]);
    /// let b = a.submatrix(2, 2);
    /// ```

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut values = [[0.0; 4]; 4];
        let width = self.width - 1;
        let height = self.height - 1;

        for (r, iter_row) in values.iter_mut().enumerate().take(width) {
            for (c, iter_item) in iter_row.iter_mut().enumerate().take(height) {
                let rx = if r < row { r } else { r + 1 };

                let cx = if c < col { c } else { c + 1 };

                *iter_item = self[rx][cx];
            }
        }

        Matrix {
            width,
            height,
            data: values,
        }
    }

    /// Returns a determinant of a matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use ::raytracer::units::Matrix;
    /// let a = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);
    /// let b = a.determinant();
    /// ```

    pub fn determinant(&self) -> f64 {
        // match self.width {
        //     2 => self[0][0] * self[1][1] - self[0][1] * self[1][0],
        //     _ => (0..self.width).fold(0.0, |result, c| result + self[0][c] * self.cofactor(0, c)),
        // }
        if self.width == 2 {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        } else {
            let mut res = 0.0;
            for r in 0..self.height {
                res += self[0][r] * self.cofactor(0, r)
            }
            res
        }
    }

    /// Returns a cofactor of matrix
    ///
    /// # Arguments
    ///
    /// * `row` - a row where to find determinant
    /// * `col` - a column where to find determinant
    ///
    /// # Examples
    /// ```
    /// use ::raytracer::units::Matrix;
    ///let a = Matrix::from([
    ///         [3.0, 5.0, 0.0],
    ///         [2.0, -1.0, -7.0],
    ///         [6.0, -1.0, 5.0],
    ///      ]);
    /// let b = a.cofactor(2, 2);
    /// ```

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        // let sign = if row + col % 2 == 1 { -1.0 } else { 1.0 };
        // // println!("{:?}", self.minor(row, col));

        // sign * self.minor(row, col)
        let minor = self.minor(row, col);

        match (row + col) % 2 {
            0 => minor,
            _ => -minor,
        }
    }

    /// Returns a minor of a given submatrix
    ///
    /// # Arguments
    ///
    /// `row` - row to remove
    /// `col` - column to remove
    ///
    /// # Example
    ///
    /// ```
    /// use ::raytracer::units::Matrix;
    ///
    /// let a = Matrix::from([
    ///         [3.0, 5.0, 0.0],
    ///         [2.0, -1.0, -7.0],
    ///         [6.0, -1.0, 5.0],
    ///     ]);
    /// let b = a.minor(2, 2);
    /// ```

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    /// Returns inverted Matrix.
    /// If matrix is impossible to invert, it retuns an Error
    ///
    /// # Example
    ///
    /// ```
    /// use ::raytracer::units::Matrix;
    /// let a = Matrix::from([
    ///         [3.0, 5.0, 0.0],
    ///         [2.0, -1.0, -7.0],
    ///         [6.0, -1.0, 5.0],
    ///     ]);
    /// let b = a.invert().unwrap();
    ///
    /// ```

    pub fn invert(&self) -> Result<Matrix, &'static str> {
        if self.determinant() == 0.0 {
            return Err("Matrix is impossible to invert");
        }

        let mut return_matrix = Matrix::new(self.width, self.height);

        for row in 0..self.height {
            for col in 0..self.width {
                let cofactor = self.cofactor(row, col);
                return_matrix.data[col][row] = cofactor / self.determinant();
            }
        }
        Ok(return_matrix)
    }
    /// Returns a matrix that when multiplied by tuple, translates it
    /// by passed values.
    ///
    /// # Arguments
    ///
    /// * `x` - x translation distance
    /// * `y` - y translation distance
    /// * `z` - z translation distance
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    pub fn translate<T: Into<f64>>(x: T, y: T, z: T) -> Matrix {
        let mut return_matrix = IDENTITY_MATRIX;
        return_matrix[0][3] = x.into();
        return_matrix[1][3] = y.into();
        return_matrix[2][3] = z.into();
        return_matrix
    }

    /// Returns a matrix that when multiplied by tuple, scales it
    /// by passed values
    /// # Arguments
    ///
    /// * `x` - x scalar
    /// * `y` - y scalar
    /// * `z` - z scalar
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    pub fn scale<T: Into<f64>>(x: T, y: T, z: T) -> Matrix {
        let mut return_matrix = IDENTITY_MATRIX;
        return_matrix[0][0] = x.into();
        return_matrix[1][1] = y.into();
        return_matrix[2][2] = z.into();
        return_matrix
    }

    /// Returns a matrix that when multiplied by tuple, rotates it
    /// on x axis.
    ///
    /// # Arguments
    ///
    /// * `r` - radians
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    pub fn rotate_x<T: Into<f64> + Copy>(r: T) -> Matrix {
        let mut return_matrix = IDENTITY_MATRIX;
        return_matrix[1][1] = r.into().cos();
        return_matrix[1][2] = -1.0 * r.into().sin();
        return_matrix[2][1] = r.into().sin();
        return_matrix[2][2] = r.into().cos();
        return_matrix
    }

    /// Returns a matrix that when multiplied by tuple, rotates it
    /// on y axis.
    ///
    /// # Arguments
    ///
    /// * `r` - radians
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    pub fn rotate_y<T: Into<f64> + Copy>(r: T) -> Matrix {
        let mut return_matrix = IDENTITY_MATRIX;
        return_matrix[0][0] = r.into().cos();
        return_matrix[0][2] = r.into().sin();
        return_matrix[2][0] = -1.0 * r.into().sin();
        return_matrix[2][2] = r.into().cos();
        return_matrix
    }

    /// Returns a matrix that when multiplied by tuple, rotates it
    /// on z axis.
    ///
    /// # Arguments
    ///
    /// * `r` - radians
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    pub fn rotate_z<T: Into<f64> + Copy>(r: T) -> Matrix {
        let mut return_matrix = IDENTITY_MATRIX;
        return_matrix[0][0] = r.into().cos();
        return_matrix[0][1] = -1.0 * r.into().sin();
        return_matrix[1][0] = r.into().sin();
        return_matrix[1][1] = r.into().cos();
        return_matrix
    }

    /// Returns a matrix that when multiplied by tuple, skews it.
    ///
    /// # Arguments
    ///
    /// * `x_to_y` - x to y skew amount
    /// * `x_to_z` - x to z skew amount
    /// * `y_to_x` - y to x skew amount
    /// * `y_to_z` - y to z skew amount
    /// * `z_to_x` - z to x skew amount
    /// * `z_to_y` - z to y skew amount

    /// # Examples
    /// ```
    ///
    /// ```
    pub fn skew<T: Into<f64> + Copy>(
        x_to_y: T,
        x_to_z: T,
        y_to_x: T,
        y_to_z: T,
        z_to_x: T,
        z_to_y: T,
    ) -> Matrix {
        let mut return_matrix = IDENTITY_MATRIX;
        return_matrix[0][1] = x_to_y.into();
        return_matrix[0][2] = x_to_z.into();

        return_matrix[1][0] = y_to_x.into();
        return_matrix[1][2] = y_to_z.into();

        return_matrix[2][0] = z_to_x.into();
        return_matrix[2][1] = z_to_y.into();

        return_matrix
    }

    /// Transforms the view according to given parameters
    pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);
        let orientation = Matrix::from([
            [left.x, left.y, left.z, 0.],
            [true_up.x, true_up.y, true_up.z, 0.],
            [-forward.x, -forward.y, -forward.z, 0.],
            [0., 0., 0., 1.],
        ]);
        orientation * Matrix::translate(-from.x, -from.y, -from.z)
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        if other.width != self.width {
            panic!("Different matrix sizes!");
        }
        let mut out = Matrix::new(self.width, self.height);
        // let mut x = 0;
        for i in 0..self.width {
            for j in 0..self.height {
                for k in 0..other.width {
                    out[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        out
    }
}

impl ops::Mul<Point> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Point) -> Matrix {
        let mut return_matrix = Matrix::new(4, 4);
        if self.width != 4 {
            panic!("Oof");
        }
        let tuple_matrix = Matrix::from([
            [other.get_x(), 0., 0., 0.],
            [other.get_y(), 0., 0., 0.],
            [other.get_z(), 0., 0., 0.],
            [other.get_w(), 0., 0., 0.],
        ]);

        for row in 0..self.height {
            for value in 0..self.width {
                return_matrix.data[row][0] += self[row][value] * tuple_matrix[value][0];
            }
        }
        return_matrix
    }
}

impl ops::Mul<Vector> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Vector) -> Matrix {
        let mut return_matrix = Matrix::new(4, 4);
        if self.width != 4 {
            panic!("Oof");
        }
        let tuple_matrix = Matrix::from([
            [other.get_x(), 0., 0., 0.],
            [other.get_y(), 0., 0., 0.],
            [other.get_z(), 0., 0., 0.],
            [other.get_w(), 0., 0., 0.],
        ]);

        for row in 0..self.height {
            for value in 0..self.width {
                return_matrix.data[row][0] += self[row][value] * tuple_matrix[value][0];
            }
        }
        return_matrix
    }
}

impl ops::Index<usize> for Matrix {
    type Output = [f64; 4];
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.height {
            panic!("Biggest oof")
        }
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl From<[[f64; 2]; 2]> for Matrix {
    fn from(array: [[f64; 2]; 2]) -> Matrix {
        let mut m = Matrix::new(2, 2);
        for x in 0..2 {
            m[x][..2].clone_from_slice(&array[x][..2]);
        }
        m
    }
}

impl From<[[f64; 3]; 3]> for Matrix {
    fn from(array: [[f64; 3]; 3]) -> Matrix {
        let mut m = Matrix::new(3, 3);
        for x in 0..3 {
            m[x][..3].clone_from_slice(&array[x][..3]);
        }
        m
    }
}
impl From<[[f64; 4]; 4]> for Matrix {
    fn from(array: [[f64; 4]; 4]) -> Matrix {
        let mut m = Matrix::new(4, 4);
        for x in 0..4 {
            m[x][..4].clone_from_slice(&array[x][..4])
        }
        m
    }
}

pub trait Transformable {
    fn transform(&self, transformation_matrix: Matrix) -> Self;
    fn translate<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self;
    fn scale<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self;
    fn rotate_x<T: Into<f64> + Copy>(&self, r: T) -> Self;
    fn rotate_y<T: Into<f64> + Copy>(&self, r: T) -> Self;
    fn rotate_z<T: Into<f64> + Copy>(&self, r: T) -> Self;
    fn skew<T: Into<f64> + Copy>(
        &self,
        x_to_y: T,
        x_to_z: T,
        y_to_x: T,
        y_to_z: T,
        z_to_x: T,
        z_to_y: T,
    ) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m[0][0], 1.0);
    }
    #[test]
    fn cmp() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(a, b);

        let c = Matrix::from([
            [3.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert!(a != c);
    }

    #[test]
    fn multiply_by_matrix() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let c = Matrix::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(a * b, c);
    }
    #[test]
    fn multiply_by_tuple() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Point::new(1, 2, 3);
        let c = Matrix::from([
            [18.0, 0., 0., 0.],
            [24.0, 0., 0., 0.],
            [33.0, 0., 0., 0.],
            [1.0, 0., 0., 0.],
        ]);
        assert_eq!(a * b, c);
    }
    #[test]
    fn transpose() {
        let a = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ])
        .transpose();

        let b = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(a, b);
    }
    #[test]
    fn determinant() {
        let a = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn submatrix() {
        let a = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let s = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);
        assert_eq!(a.submatrix(0, 2), s);

        let a = Matrix::from([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.],
        ]);
        let s = Matrix::from([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);
        assert_eq!(a.submatrix(2, 1), s);
    }

    #[test]
    fn minor() {
        let a = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor() {
        let a = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn invert() {
        let a = Matrix::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        let b = a.invert().unwrap();
        assert_eq!(b[0][0], -0.040740740740740744);
    }

    #[test]
    fn view_transform() {
        // The transformation matrix for the default orientation
        let t = Matrix::view_transform(
            Point::new(0, 0, 0),
            Point::new(0, 0, -1),
            Vector::new(0, 1, 0),
        );

        assert_eq!(t, IDENTITY_MATRIX);

        let t = Matrix::view_transform(
            Point::new(0, 0, 8),
            Point::new(0, 0, 0),
            Vector::new(0, 1, 0),
        );

        assert_eq!(t, Matrix::translate(0, 0, -8));
    }
}
