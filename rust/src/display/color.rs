use crate::helper::almost;
use num::NumCast;
use std::cmp::{max, min};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub const WHITE: Color = Color {
        red: 1.,
        green: 1.,
        blue: 1.,
    };
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const RED: Color = Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const LIGHT_GREEN: Color = Color {
        red: 0.1,
        green: 1.0,
        blue: 0.5
    };
    pub const MUSTARD_YELLOW: Color = Color {
        red: 1.0,
        green: 0.8,
        blue: 0.1
    };
    pub const BLUE: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    };

    pub fn new<T: NumCast>(red: T, green: T, blue: T) -> Self {
        Color {
            red: red.to_f64().unwrap(),
            green: green.to_f64().unwrap(),
            blue: blue.to_f64().unwrap(),
        }
    }

    pub fn to255(self) -> (u64, u64, u64) {
        (
            value_to_255(self.red),
            value_to_255(self.green),
            value_to_255(self.blue),
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        almost(self.red, other.red)
            && almost(self.green, other.green)
            && almost(self.blue, other.blue)
    }
}
impl Eq for Color {}

fn value_to_255(value: f64) -> u64 {
    min(255, max(0, (value * 255.0).round() as u64))
}
