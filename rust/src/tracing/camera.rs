use crate::display::canvas::Canvas;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::tracing::ray::Ray;

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

    fn rayForPixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canas to the pixel's center
        let xOffset = (x as f64 + 0.5) * self.pixel_size();
        let yOffset = (y as f64 + 0.5) * self.pixel_size();

        // the untransformed coordinates of the pixel in world space
        // (remember that the camera looks toward -z, so +x is to the left)
        let worldX = self.calculate_half_width() - xOffset;
        let worldY = self.calculate_half_height() - yOffset;

        // using the camera matrix, transform the canvas point and the origin
        // and then compute the ray's direction vector
        // remember that the canvas is at z=-1
        let pixel = &self.transform.inverse() * Point::at(worldX, worldY, -1.0);
        let origin = &self.transform.inverse() * Point::origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    fn pixel_size(&self) -> f64 {
        self.calculate_half_width() * 2.0 / self.hsize as f64
    }

    fn calculate_half_height(&self) -> f64 {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;

        return if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };
    }

    fn calculate_half_width(&self) -> f64 {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;

        return if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{rotation_y, translation};
    use crate::geometry::vector::Vector;
    use crate::helper::almost;
    use crate::tracing::camera::Camera;
    use std::f64::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let camera = Camera::new(hsize, vsize, field_of_view, Matrix::identity(4));

        assert_eq!(160, camera.hsize);
        assert_eq!(120, camera.vsize);
        assert_eq!(PI / 2.0, camera.field_of_view);
        assert_eq!(Matrix::identity(4), camera.transform);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let camera = Camera::new(200, 125, PI / 2.0, Matrix::identity(4));
        assert!(almost(0.01, camera.pixel_size()));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let camera = Camera::new(125, 200, PI / 2.0, Matrix::identity(4));
        assert!(almost(0.01, camera.pixel_size()));
    }

    #[test]
    fn constructing_ray_through_center_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0, Matrix::identity(4));

        let ray = camera.rayForPixel(100, 50);

        assert_eq!(Point::origin(), ray.position(0));
        assert_eq!(Vector::new(0, 0, -1), ray.direction());
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0, Matrix::identity(4));

        let ray = camera.rayForPixel(0, 0);

        assert_eq!(Point::origin(), ray.position(0));
        assert_eq!(Vector::new(0.66519, 0.33259, -0.66851), ray.direction());
    }

    #[test]
    fn constructing_ray_when_the_camera_is_transformed() {
        let transform = translation(0, -2, 5).then(&rotation_y(PI / 4.0));
        let camera = Camera::new(201, 101, PI / 2.0, transform);

        let ray = camera.rayForPixel(100, 50);

        assert_eq!(Point::at(0, 2, -5), ray.position(0));
        assert_eq!(
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0),
            ray.direction()
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        assert!(false)
        // let world = World.default();
        //
        // let from = Point::at(0, 0, -5);
        // let to = WORLD_ORIGIN;
        // let up = Vector::new(0, 1, 0);
        // let camera = Camera(11, 11, Math.PI / 2, viewTransform(from, to, up));
        //
        // let image = camera.render(world);
        // assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.pixelAt(5, 5));
    }
}
