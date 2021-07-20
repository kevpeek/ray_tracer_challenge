use crate::display::canvas::Canvas;
use crate::display::color::Color;
use crate::display::writer::write_canvas;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;

pub fn run() {
    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let environment = (gravity, wind);
    let projectile = (
        Point::at(0.0, 0.0, 0.0),
        Vector::new(1.0, 1.0, 0.0).normalize(),
    );

    let points = iterate(vec![projectile], environment);

    let width = 200;
    let height = 200;
    let mut canvas = Canvas::new(width, height);
    points
        .iter()
        .map(|(point, _)| *point)
        .map(|point| ((point.x * 10.0) as usize, (point.y * 10.0) as usize))
        .map(|(x, y)| (x, height - y - 1))
        .for_each(|(x, y)| canvas.write_pixel(x, y, Color::RED));

    write_canvas(&canvas).unwrap();
}

fn iterate(mut path: Vec<(Point, Vector)>, environment: (Vector, Vector)) -> Vec<(Point, Vector)> {
    let (location, velocity) = path.last().unwrap();
    let (gravity, wind) = environment;

    let new_location = *location + *velocity;
    if (new_location.y < 0.0) {
        return path;
    }

    let new_velocity = *velocity + gravity + wind;
    path.push((new_location, new_velocity));
    iterate(path, environment)
}
