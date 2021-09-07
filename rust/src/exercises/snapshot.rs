use crate::display::canvas::Canvas;
pub use crate::display::ppm_writer;
use crate::display::resolution::Resolution;
use crate::geometry::point::Point;
use crate::geometry::transformations::view_transform;
use crate::geometry::vector::Vector;
use crate::tracing::camera::Camera;
use crate::tracing::world::World;
use image::RgbImage;
use std::f64::consts::PI;

pub fn snapshot_world(world: World, camera: Camera) {
    let canvas = camera.render(world);
    write_jpg("output.jpg", canvas);
}

pub fn make_camera_one(resolution: Resolution) -> Camera {
    let camera_transform = view_transform(
        Point::at(0.0, 1.5, -5.0),
        Point::at(0, 1, 0),
        Vector::new(0, 1, 0),
    );
    Camera::new(resolution, PI / 3.0, camera_transform)
}

fn write_jpg(file_name: &str, canvas: Canvas) {
    let mut image_buffer: RgbImage =
        image::ImageBuffer::new(canvas.width as u32, canvas.height as u32);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let (red, green, blue) = canvas.pixel_at(x as usize, y as usize).to255();
        *pixel = image::Rgb([red as u8, green as u8, blue as u8]);
    }

    image_buffer.save(file_name).unwrap();
}
