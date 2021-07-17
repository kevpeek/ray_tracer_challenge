use std::cmp::{min, max};
use std::ops::{Add, Sub, Mul};
use crate::helper::almost;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0 };
    pub const RED: Color = Color {red: 1.0,green:  0.0,blue: 0.0 };
    pub const GREEN: Color = Color {red: 0.0,green:  1.0,blue: 0.0 };
    pub const BLUE: Color = Color {red: 0.0,green:  0.0,blue: 1.0 };
    pub const WHITE: Color = Color { red: 1., green: 1., blue: 1.};

    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color {red, green, blue}
    }

    pub fn to255(self) -> (u64, u64, u64) {
        (value_to_255(self.red), value_to_255(self.green), value_to_255(self.blue))
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue)
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.red * rhs.red, self.green * rhs.green, self.blue * rhs.blue)
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
        almost(self.red, other.red) && almost(self.green, other.green) && almost(self.blue, other.blue)
    }
}
impl Eq for Color {}

fn value_to_255(value: f64) -> u64 {
    min(255, max(0, (value * 255.0).round() as u64))
}
