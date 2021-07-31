use crate::helper::enumerate_coordinates;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Resolution {
    hsize: usize,
    vsize: usize,
}

impl Resolution {
    pub const LOW: Resolution = Resolution {
        hsize: 400,
        vsize: 200,
    };
    pub const FHD: Resolution = Resolution {
        hsize: 1920,
        vsize: 1080,
    };
    pub fn new(hsize: usize, vsize: usize) -> Resolution {
        Resolution { hsize, vsize }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn aspect(&self) -> f64 {
        self.hsize as f64 / self.vsize as f64
    }

    pub fn coordinates(&self) -> Vec<(usize, usize)> {
        enumerate_coordinates(0..self.hsize, 0..self.vsize)
    }
}
