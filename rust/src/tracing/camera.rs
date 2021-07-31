use crate::display::canvas::Canvas;
use crate::display::color::Color;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::helper::enumerate_coordinates;
use crate::tracing::ray::Ray;
use crate::tracing::world::World;
use rayon::prelude::*;

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

    fn aspect(&self) -> f64 {
        self.hsize as f64 / self.vsize as f64
    }

    fn coordinates(&self) -> Vec<(usize, usize)> {
        enumerate_coordinates(0..self.hsize, 0..self.vsize)
    }
}

pub struct Camera {
    resolution: Resolution,
    field_of_view: f64,
    transform: Matrix,
    half_height: f64,
    half_width: f64,
}

impl Camera {
    pub fn new(resolution: Resolution, field_of_view: f64, transform: Matrix) -> Camera {
        Camera {
            resolution,
            field_of_view,
            transform,
            half_height: Camera::calculate_half_height(resolution, field_of_view),
            half_width: Camera::calculate_half_width(resolution, field_of_view),
        }
    }

    /**
     * Produce the image of the world as seen from this camera.
     */
    pub(crate) fn render(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.resolution.hsize, self.resolution.vsize);
        let pixels: Vec<(usize, usize, Color)> = self
            .resolution
            .coordinates()
            .par_iter()
            .map(|(x, y)| (*x, *y, self.ray_for_pixel(*x, *y)))
            .map(|(x, y, ray)| (x, y, world.color_at(&ray)))
            .collect();
        pixels
            .into_iter()
            .for_each(|(x, y, color)| canvas.write_pixel(x, y, color));

        canvas
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let x_offset = (x as f64 + 0.5) * self.pixel_size();
        let y_offset = (y as f64 + 0.5) * self.pixel_size();

        // the untransformed coordinates of the pixel in world space
        // (remember that the camera looks toward -z, so +x is to the left)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // using the camera matrix, transform the canvas point and the origin
        // and then compute the ray's direction vector
        // remember that the canvas is at z=-1
        let pixel = &self.transform.inverse() * Point::at(world_x, world_y, -1.0);
        let origin = &self.transform.inverse() * Point::origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    fn pixel_size(&self) -> f64 {
        self.half_width * 2.0 / self.resolution.hsize as f64
    }

    fn calculate_half_height(resolution: Resolution, field_of_view: f64) -> f64 {
        let half_view = (field_of_view / 2.0).tan();

        if resolution.aspect() >= 1.0 {
            half_view / resolution.aspect()
        } else {
            half_view
        }
    }

    fn calculate_half_width(resolution: Resolution, field_of_view: f64) -> f64 {
        let half_view = (field_of_view / 2.0).tan();

        if resolution.aspect() >= 1.0 {
            half_view
        } else {
            half_view * resolution.aspect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{rotation_y, translation, view_transform};
    use crate::geometry::vector::Vector;
    use crate::helper::almost;
    use crate::tracing::camera::{Camera, Resolution};
    use crate::tracing::world::World;
    use std::f64::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let resolution = Resolution::new(160, 120);
        let field_of_view = PI / 2.0;

        let camera = Camera::new(resolution, field_of_view, Matrix::identity(4));

        assert_eq!(resolution, camera.resolution);
        assert_eq!(PI / 2.0, camera.field_of_view);
        assert_eq!(Matrix::identity(4), camera.transform);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let camera = Camera::new(Resolution::new(200, 125), PI / 2.0, Matrix::identity(4));
        assert!(almost(0.01, camera.pixel_size()));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let camera = Camera::new(Resolution::new(125, 200), PI / 2.0, Matrix::identity(4));
        assert!(almost(0.01, camera.pixel_size()));
    }

    #[test]
    fn constructing_ray_through_center_of_the_canvas() {
        let camera = Camera::new(Resolution::new(201, 101), PI / 2.0, Matrix::identity(4));

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(Point::origin(), ray.position(0));
        assert_eq!(Vector::new(0, 0, -1), ray.direction());
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let camera = Camera::new(Resolution::new(201, 101), PI / 2.0, Matrix::identity(4));

        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(Point::origin(), ray.position(0));
        assert_eq!(Vector::new(0.66519, 0.33259, -0.66851), ray.direction());
    }

    #[test]
    fn constructing_ray_when_the_camera_is_transformed() {
        let transform = translation(0, -2, 5).then(&rotation_y(PI / 4.0));
        let camera = Camera::new(Resolution::new(201, 101), PI / 2.0, transform);

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(Point::at(0, 2, -5), ray.position(0));
        assert_eq!(
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0),
            ray.direction()
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        let world = World::default();

        let from = Point::at(0, 0, -5);
        let to = Point::origin();
        let up = Vector::new(0, 1, 0);
        let camera = Camera::new(
            Resolution::new(11, 11),
            PI / 2.0,
            view_transform(from, to, up),
        );

        let image = camera.render(world);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.pixel_at(5, 5));
    }
}
