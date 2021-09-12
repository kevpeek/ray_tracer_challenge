use std::ops::Range;

pub fn enumerate_coordinates(xs: Range<usize>, ys: Range<usize>) -> Vec<(usize, usize)> {
    xs.flat_map(|x| ys.clone().map(move |y| (x, y))).collect()
}
