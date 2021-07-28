use crate::geometry::matrix::Matrix;
use crate::geometry::vector::Vector;
use crate::helper::almost;
use num::NumCast;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn origin() -> Point {
        Point::at(0.0, 0.0, 0.0)
    }

    pub fn at<T: NumCast>(x: T, y: T, z: T) -> Self {
        Point {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
            z: z.to_f64().unwrap(),
        }
    }

    // By convention, a Point is treated as a 4x1 Matrix with a 4th element of 1.
    pub fn as_matrix(&self) -> Matrix {
        Matrix::of_size(4, 1).of(vec![self.x, self.y, self.z, 1.0])
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::at(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::at(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        almost(self.x, other.x) && almost(self.y, other.y) && almost(self.z, other.z)
    }
}
impl Eq for Point {}

#[cfg(test)]
mod tests {
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;

    #[test]
    fn test_equals_almost() {
        let one = Point::at(1, 2, 3);
        let two = Point {
            x: 1.000009,
            y: 2.000009,
            z: 3.000009,
        };
        assert_eq!(one, two);
    }

    #[test]
    fn test_equals_different() {
        let one = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let two = Point {
            x: 1.00001,
            y: 2.00001,
            z: 3.00001,
        };
        assert_ne!(one, two);
    }

    #[test]
    fn test_add() {
        let result = Point::origin() + Vector::new(1.0, 2.0, 3.0);
        assert_eq!(result, Point::at(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_sub_vector() {
        let point = Point::at(2.0, 4.0, 6.0);
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(Point::at(1.0, 2.0, 3.0), point - vector);
    }

    #[test]
    fn test_sub_point() {
        let point1 = Point::at(2.0, 4.0, 6.0);
        let point2 = Point::at(1.0, 2.0, 3.0);
        assert_eq!(Vector::new(1.0, 2.0, 3.0), point1 - point2);
    }

    #[test]
    fn test_to_matrix() {
        let point = Point::at(9.0, 8.0, 7.0);
        assert_eq!(
            Matrix::of_size(4, 1).of(vec![9, 8, 7, 1]),
            point.as_matrix()
        );
    }
}
