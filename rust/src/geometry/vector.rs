use crate::geometry::matrix::Matrix;
use crate::helpers::approximate::Approximate;
use num::NumCast;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

const ZERO: Vector = Vector {
    x: 0.,
    y: 0.,
    z: 0.,
};

impl Vector {
    pub fn new<T: NumCast>(x: T, y: T, z: T) -> Vector {
        Vector {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
            z: z.to_f64().unwrap(),
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }

    pub fn as_matrix(&self) -> Matrix {
        Matrix::of_size(4, 1).of(vec![self.x, self.y, self.z, 0.0])
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ZERO - self
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.almost(other.x) && self.y.almost(other.y) && self.z.almost(other.z)
    }
}
impl Eq for Vector {}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector;

    #[test]
    fn test_equals_almost() {
        let one = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let two = Vector {
            x: 1.000009,
            y: 2.000009,
            z: 3.000009,
        };
        assert_eq!(one, two);
    }

    #[test]
    fn test_equals_different() {
        let one = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let two = Vector {
            x: 1.00001,
            y: 2.00001,
            z: 3.00001,
        };
        assert_ne!(one, two);
    }

    #[test]
    fn test_add() {
        let one = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let two = Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let expected = Vector {
            x: 5.0,
            y: 7.0,
            z: 9.0,
        };
        assert_eq!(one + two, expected);
    }

    #[test]
    fn test_sub() {
        let one = Vector {
            x: 5.0,
            y: 7.0,
            z: 9.0,
        };
        let two = Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let expected = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(one - two, expected);
    }

    #[test]
    fn test_neg() {
        let one = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected = Vector {
            x: -1.0,
            y: -2.0,
            z: -3.0,
        };
        assert_eq!(-one, expected);
    }

    #[test]
    fn test_mul() {
        let one = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected = Vector {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        assert_eq!(one * 2.0, expected);
    }

    #[test]
    fn test_div() {
        let one = Vector {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        let expected = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(one / 2.0, expected);
    }

    #[test]
    fn test_magnitude() {
        let one = Vector {
            x: 4.0,
            y: 4.0,
            z: 2.0,
        };
        assert_eq!(one.magnitude(), 6.0);
    }

    #[test]
    fn test_normalize() {
        let vector = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let sqrt14 = 14.0_f32.sqrt() as f64;
        assert_eq!(
            Vector {
                x: 1.0 / sqrt14,
                y: 2.0 / sqrt14,
                z: 3.0 / sqrt14
            },
            vector.normalize()
        );
    }

    #[test]
    fn test_dot() {
        let vector1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vector2 = Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(20.0, vector1.dot(vector2));
    }

    #[test]
    fn test_cross() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);

        let cross12 = vector1.cross(vector2);
        assert_eq!(Vector::new(-1.0, 2.0, -1.0), cross12);

        let cross21 = vector2.cross(vector1);
        assert_eq!(Vector::new(1.0, -2.0, 1.0), cross21);
    }

    #[test]
    fn test_reflect() {
        let vector = Vector::new(1., -1., 0.);
        let normal = Vector::new(0., 1., 0.);

        let reflection = vector.reflect(normal);

        assert_eq!(Vector::new(1., 1., 0.), reflection);
    }
}
