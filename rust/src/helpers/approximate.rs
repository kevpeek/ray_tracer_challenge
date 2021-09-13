pub const EPSILON: f64 = 0.00001;

pub trait Approximate {
    fn almost(&self, other: f64) -> bool;
    fn almost_zero(&self) -> bool {
        self.almost(0.0)
    }
}

impl Approximate for f64 {
    fn almost(&self, other: Self) -> bool {
        (self - other).abs() < EPSILON
    }
}
