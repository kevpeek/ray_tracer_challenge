use crate::geometry::matrix::Matrix;

mod camera;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, transform: Matrix) -> Camera {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
        }
    }
}
