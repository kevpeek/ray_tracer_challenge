use crate::helper::{almost, enumerate_coordinates};
use std::ops::{Index, Mul};
use num::{NumCast, Integer};
use crate::geometry::vector::Vector;
use crate::geometry::point::Point;

#[derive(Debug)]
pub struct Matrix {
    height: usize,
    width: usize,
    values: Vec<f64>,
}

// fn get_index_for(&self, row: usize, column: usize) -> usize {
//     row * self.width + column
// }
impl Matrix {
    pub fn of_size(height: usize, width: usize) -> MatrixBuilder {
        MatrixBuilder {height, width}
    }

    pub fn square(size: usize) -> MatrixBuilder {
        Matrix::of_size(size, size)
    }

    pub fn identity(size: usize) -> Matrix {
        let values = enumerate_coordinates(0..size, 0..size).iter()
            .map(|(row, col)| if row == col {1.0} else {0.0})
            .collect();
        Matrix {width: size, height: size, values }
    }


    fn get_row(&self, i: usize) -> &[f64] {
        &self.values[self.width * i..self.width * i + self.width]
    }

    fn get_column(&self, column_index: usize) -> Vec<f64> {
        (0..self.height).map(|row_index| self[row_index][column_index]).collect()
    }

    fn transpose(&self) -> Matrix {
        Matrix {
            width: self.width,
            height: self.height,
            values: (0..self.width).flat_map(|column| self.get_column(column)).collect()
        }
    }

    fn determinant(&self) -> f64 {
        /*
        fun determinant(): Double = when {
        height == 2 && width == 2 -> this[0, 0] * this[1, 1] - this[0, 1] * this[1, 0]
        else -> getRow(0).mapIndexed { column, value -> cofactor(0, column) * value }.sum()
    }
         */
        if self.height == 2 && self.width == 2 {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        } else {
            self.get_row(0).iter().enumerate()
                .map(|(column, value)| self.cofactor(0, column) * value).sum()
        }
    }

    /**
     * Returns the sub matrix created by removing the specified row and column.
     */
    fn submatrix(&self, row: usize, column: usize) -> Matrix {
        let values_to_keep = enumerate_coordinates(0..self.height, 0..self.width).iter()
            // calculate the indexes to keep
            .filter(|(r, c)| *r != row && *c != column)
            // then get those values
            .map(|(r, c)| self[*r][*c]).collect();
        Matrix::of_size(self.height -1, self.width - 1).of(values_to_keep)
    }

    fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        self.minor(row, column) * self.cofactor_sign(row, column)
    }

    fn cofactor_sign(&self, row: usize, column: usize) -> f64 {
        if (row + column).is_even() { 1.0 } else { -1.0 }
    }

    fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn inverse(&self) -> Matrix {
        if !self.invertible() {
            panic!("Attempt to invert a non-invertible matrix{:?}", self);
        }
        let determinant = self.determinant();
        let inverse_values: Vec<f64> = enumerate_coordinates(0..self.width, 0..self.height).iter()
            .map(|(c, r)| self.cofactor(*r, *c) / determinant).collect();
        Matrix::of_size(self.height, self.width).of(inverse_values)
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, i: usize) -> &Self::Output {
        self.get_row(i)
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Self::Output {
        let new_values: Vec<f64> = (0..self.height).flat_map(|row_index| {
            let row = self.get_row(row_index);
            (0..rhs.width).map(move |column_index| {
                let column = rhs.get_column(column_index);
                row.iter().zip(column).map(|(a,b)| a * b).sum()
            })
        }).collect();
        Matrix {height: self.height, width: rhs.width, values: new_values}
    }
}

impl Mul<Point> for &Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let result = self * &rhs.as_matrix();
        Point::at(result[0][0], result[1][0], result[2][0])
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let result = self * &rhs.as_matrix();
        Vector::new(result[0][0], result[1][0], result[2][0])
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.values.iter().zip(other.values.iter()).all(|(a, b)| almost(*a, *b))
    }
}
impl Eq for Matrix {}

pub struct MatrixBuilder {
    width: usize,
    height: usize,
}

impl MatrixBuilder {
    pub fn of<T: NumCast>(self, values: Vec<T>) -> Matrix {
        let values = values.iter()
            .map(|n| n.to_f64().unwrap())
            .collect();
        Matrix {width: self.width,height: self.height, values }
    }
}


#[cfg(test)]
mod tests {
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;

    #[test]
    fn test_create_and_access() {
        let matrix = Matrix::square(2).of(vec![1, 2, 3, 4]);
        assert_eq!(1.0, matrix[0][0]);
        assert_eq!(2.0, matrix[0][1]);
        assert_eq!(3.0, matrix[1][0]);
        assert_eq!(4.0, matrix[1][1]);
    }

    #[test]
    fn test_equals_almost() {
        let one = Matrix::square(2).of(vec![1.1, 2.2, 3.3, 4.4]);
        let two = Matrix::square(2).of(vec![1.1, 2.2, 3.3, 4.4]);
        assert_eq!(one, two);
    }

    #[test]
    fn test_not_equals() {
        let matrixA = Matrix::of_size(4, 4).of(vec![
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        ]);

        let matrixB = Matrix::of_size(4, 4).of(vec![
            2, 3, 4, 5,
            6, 7, 8, 9,
            8, 7, 6, 5,
            4, 3, 2, 1
        ]);

        assert_ne!(matrixA, matrixB)
    }

    #[test]
    fn test_multiplication_by_identity() {
        let one = Matrix::square(2).of(vec![1.1, 2.2, 3.3, 4.4]);
        let two = Matrix::identity(2);
        assert_eq!(one, &one * &two);
    }

    #[test]
    fn test_multiplication_todo() {
        let matrixA = Matrix::of_size(4, 4).of(vec![
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        ]);

        let matrixB = Matrix::of_size(4, 4).of(vec![
            -2, 1, 2, 3,
            3, 2, 1, -1,
            4, 3, 6, 5,
            1, 2, 7, 8
        ]);

        let expectedResult = Matrix::of_size(4, 4).of(vec![
            20, 22, 50, 48,
            44, 54, 114, 108,
            40, 58, 110, 102,
            16, 26, 46, 42
        ]);

        assert_eq!(expectedResult, &matrixA * &matrixB)
    }

    #[test]
    fn test_mul_by_point() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            1, 2, 3, 4,
            2, 4, 4, 2,
            8, 6, 4, 1,
            0, 0, 0, 1
        ]);

        let point = Point::at(1.0, 2.0, 3.0);

        assert_eq!(Point::at(18.0, 24.0, 33.0), &matrix * point)
    }

    #[test]
    fn test_mul_by_vector() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            1, 2, 3, 4,
            2, 4, 4, 2,
            8, 6, 4, 1,
            0, 0, 0, 1
        ]);

        let vector = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(Vector::new(14.0, 22.0, 32.0), &matrix * vector)
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            0, 9, 3, 0,
            9, 8, 0, 8,
            1, 8, 5, 3,
            0, 0, 5, 8
        ]);

        let expectedTranspose = Matrix::of_size(4, 4).of(vec![
            0, 9, 1, 0,
            9, 8, 8, 0,
            3, 0, 5, 5,
            0, 8, 3, 8
        ]);

        assert_eq!(expectedTranspose, matrix.transpose())
    }

    #[test]
    fn test_transpose_identity() {
        let identity = Matrix::identity(4);
        assert_eq!(identity, identity.transpose())
    }

    #[test]
    fn test_determinant_2x2() {
        let matrix = Matrix::of_size(2, 2).of(vec![
            1, 5,
            -3, 2
        ]);

        assert_eq!(17.0, matrix.determinant())
    }

    #[test]
    fn test_submatrix_3x3() {
        let matrix = Matrix::of_size(3, 3).of(vec![
            1, 5, 0,
            -3, 2, 7,
            0, 6, -3
        ]);

        let expectedSub = Matrix::of_size(2, 2).of(vec![
            -3, 2,
            0, 6
        ]);

        assert_eq!(expectedSub, matrix.submatrix(0, 2))
    }

    #[test]
    fn test_submatrix_4x4() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            -6, 1, 1, 6,
            -8, 5, 8, 6,
            -1, 0, 8, 2,
            -7, 1, -1, 1
        ]);

        let expectedSub = Matrix::of_size(3, 3).of(vec![
            -6, 1, 6,
            -8, 8, 6,
            -7, -1, 1
        ]);

        assert_eq!(expectedSub, matrix.submatrix(2, 1))
    }

    #[test]
    fn test_minor_3x3() {
        let matrix = Matrix::of_size(3, 3).of(vec![
            3, 5, 0,
            2, -1, -7,
            6, -1, 5
        ]);

        let submatrix = matrix.submatrix(1, 0);

        assert_eq!(25.0, submatrix.determinant());
        assert_eq!(25.0, matrix.minor(1, 0));
    }

    #[test]
    fn test_cofactor_3x3() {
        let matrix = Matrix::of_size(3, 3).of(vec![
            3, 5, 0,
            2, -1, -7,
            6, -1, 5
        ]);

        assert_eq!(-12.0, matrix.minor(0, 0));
        assert_eq!(-12.0, matrix.cofactor(0, 0));
        assert_eq!(25.0, matrix.minor(1, 0));
        assert_eq!(-25.0, matrix.cofactor(1, 0));
    }

    #[test]
    fn test_determinant_3x3() {
        let matrix = Matrix::of_size(3, 3).of(vec![
            1, 2, 6,
            -5, 8, -4,
            2, 6, 4
        ]);

        assert_eq!(56.0, matrix.cofactor(0, 0));
        assert_eq!(12.0, matrix.cofactor(0, 1));
        assert_eq!(-46.0, matrix.cofactor(0, 2));
        assert_eq!(-196.0, matrix.determinant());
    }

    #[test]
    fn test_determinant_4x4() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            -2, -8, 3, 5,
            -3, 1, 7, 3,
            1, 2, -9, 6,
            -6, 7, 7, -9
        ]);

        assert_eq!(690.0, matrix.cofactor(0, 0));
        assert_eq!(447.0, matrix.cofactor(0, 1));
        assert_eq!(210.0, matrix.cofactor(0, 2));
        assert_eq!(51.0, matrix.cofactor(0, 3));
        assert_eq!(-4071.0, matrix.determinant());
    }

    #[test]
    fn test_invertable() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            6, 4, 4, 4,
            5, 5, 7, 6,
            4, -9, 3, -7,
            9, 1, 7, -6
        ]);

        assert_eq!(-2120.0, matrix.determinant());
        assert!(matrix.invertible());
    }

    #[test]
    fn test_non_invertible() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            -4, 2, -2, -3,
            9, 6, 2, 6,
            0, -5, 1, -5,
            0, 0, 0, 0
        ]);

        assert_eq!(0.0, matrix.determinant());
        assert!(!matrix.invertible());
    }

    #[test]
    fn test_inverse() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            -5, 2, 6, -8,
            1, -5, 1, 8,
            7, 7, -6, -7,
            1, -3, 7, 4
        ]);

        let inverse = matrix.inverse();

        assert_eq!(532.0, matrix.determinant());
        assert_eq!(-160.0, matrix.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, inverse[3][2]);
        assert_eq!(105.0, matrix.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, inverse[2][3]);

        let expected_inverse = Matrix::of_size(4, 4).of(vec![
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639
        ]);

        assert_eq!(expected_inverse, inverse);
    }

    #[test]
    fn test_another_inverse() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            8, -5, 9, 2,
            7, 5, 6, 1,
            -6, 0, 9, 6,
            -3, 0, -9, -4
        ]);

        let expectedInverse = Matrix::of_size(4, 4).of(vec![
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308
        ]);

        assert_eq!(expectedInverse, matrix.inverse())
    }

    #[test]
    fn test_inverse_three() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            9, 3, 0, 9,
            -5, -2, -6, -3,
            -4, 9, 6, 4,
            -7, 6, 6, 2
        ]);

        let expectedInverse = Matrix::of_size(4, 4).of(vec![
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333
        ]);

        assert_eq!(expectedInverse, matrix.inverse())
    }

    #[test]
    fn test_multiple_product_by_inverse() {
        let matrixA = Matrix::of_size(4, 4).of(vec![
            3, -9, 7, 3,
            3, -8, 2, -9,
            -4, 4, 4, 1,
            -6, 5, -1, 1
        ]);

        let matrixB = Matrix::of_size(4, 4).of(vec![
            8, 2, 2, 2,
            3, -1, 7, 0,
            7, 0, 5, 4,
            6, -2, 0, 5
        ]);

        let matrixC = &matrixA * &matrixB;

        assert_eq!(matrixA, &matrixC * &matrixB.inverse())
    }

    #[test]
    fn test_invert_identity() {
        let identity4 = Matrix::identity(4);
        assert_eq!(identity4, identity4.inverse());
    }

    #[test]
    fn test_multiply_matrix_by_inverse() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            3, -9, 7, 3,
            3, -8, 2, -9,
            -4, 4, 4, 1,
            -6, 5, -1, 1
        ]);

        assert_eq!(Matrix::identity(4), &matrix * &matrix.inverse());
    }

    #[test]
    fn test_transpose_of_inverse_equals_inverse_of_transpose() {
        let matrix = Matrix::of_size(4, 4).of(vec![
            3, -9, 7, 3,
            3, -8, 2, -9,
            -4, 4, 4, 1,
            -6, 5, -1, 1
        ]);

        assert_eq!(matrix.transpose().inverse(), matrix.inverse().transpose());
    }
}
