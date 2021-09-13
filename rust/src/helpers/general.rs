use std::ops::Range;

pub fn enumerate_coordinates(xs: Range<usize>, ys: Range<usize>) -> Vec<(usize, usize)> {
    xs.flat_map(|x| ys.clone().map(move |y| (x, y))).collect()
}

pub trait OrderedTuple {
    fn ordered(self) -> Self;
}

impl OrderedTuple for (f64, f64) {
    fn ordered(self) -> Self {
        let (t0, t1) = self;
        if t0 <= t1 {
            (t0, t1)
        } else {
            (t1, t0)
        }
    }
}
