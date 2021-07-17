use std::ops::Range;

const EPSILON: f64 = 0.00001;
pub fn almost(a: f64, b: f64) -> bool {
    (a-b).abs() < EPSILON
}

pub fn enumerate_coordinates(xs: Range<usize>, ys: Range<usize>) -> Vec<(usize, usize)> {
    xs.flat_map(|x| ys.clone().map(move |y| (x, y))).collect()
}
