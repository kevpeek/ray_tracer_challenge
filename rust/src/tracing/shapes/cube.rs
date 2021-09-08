use crate::tracing::shapes::shape::ShapeGeometry;
use crate::tracing::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::EPSILON;
use std::cmp::{max, min};


#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Cube {}

impl Cube {
    pub fn new() -> Cube {
        Cube {}
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            let tmin = tmin_numerator / direction;
            let tmax = tmax_numerator / direction;
            (tmin, tmax)
        } else {
            let tmin = f64::MAX * tmin_numerator.signum();
            let tmax = f64::MAX * tmax_numerator.signum();
            (tmin, tmax)
        };

        if tmin <= tmax {
            (tmin, tmax)
        } else {
            (tmax, tmin)
        }
    }
}

impl ShapeGeometry for Cube {
    fn name(&self) -> &'static str {
        "cube"
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let (xmin, xmax) = self.check_axis(ray.origin().x, ray.direction().x);
        let (ymin, ymax) = self.check_axis(ray.origin().y, ray.direction().y);
        let (zmin, zmax) = self.check_axis(ray.origin().z, ray.direction().z);
        let tmin = vec![xmin, ymin, zmin].into_iter().fold(f64::NEG_INFINITY, f64::max);
        let tmax = vec![xmax, ymax, zmax].into_iter().fold(f64::INFINITY, f64::min);

        if tmin > tmax {
            return Vec::new();
        }

        vec![tmin, tmax]
    }

    fn normal_at(&self, point: Point) -> Vector {
        let max_c = point.x.abs().max(point.y.abs()).max(point.z.abs());

        return if max_c == point.x.abs() {
            Vector::new(point.x, 0.0, 0.0)
        } else if max_c == point.y.abs() {
            Vector::new(0.0, point.y, 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tracing::shapes::cube::Cube;
    use crate::tracing::ray::Ray;
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;
    use crate::tracing::shapes::shape::ShapeGeometry;

    #[test]
    fn ray_intersecting_a_cube() {
        let cases = vec![
            (Point::at(5.0, 0.5, 0.0), Vector::new(-1, 0, 0), 4.0, 6.0),
            (Point::at(-5.0, 0.5, 0.0), Vector::new(1, 0, 0), 4.0, 6.0),
            (Point::at(0.5, 5.0, 0.0), Vector::new(0, -1, 0), 4.0, 6.0),
            (Point::at(0.5, -5.0, 0.0), Vector::new(0, 1, 0), 4.0, 6.0),
            (Point::at(0.5, 0.0, 5.0), Vector::new(0, 0, -1), 4.0, 6.0),
            (Point::at(0.5, 0.0, -5.0), Vector::new(0, 0, 1), 4.0, 6.0),
            (Point::at(0.0, 0.5, 0.0), Vector::new(0, 0, 1), -1.0, 1.0),
        ];

        for (origin, direction, t1, t2) in cases {
            let cube = Cube::new();
            let ray = Ray::new(origin, direction);
            let intersections = cube.intersect(&ray);
            assert_eq!(t1, intersections[0]);
            assert_eq!(t2, intersections[1]);
        }
    }

    #[test]
    fn ray_misses_cube() {
        let cases = vec![
            (Point::at(-2, 0, 0), Vector::new(0.2673, 0.5345, 0.8018)),
            (Point::at(0, -2, 0), Vector::new(0.8018, 0.2673, 0.5345)),
            (Point::at(0, 0, -2), Vector::new(0.5345, 0.8018, 0.2673)),
            (Point::at(2, 0, 2)  , Vector::new(0, 0, -1)),
            (Point::at(0, 2, 2)  , Vector::new(0, -1, 0)),
            (Point::at(2, 2, 0)  , Vector::new(-1, 0, 0)),
        ];

        for (origin, direction) in cases {
            let cube = Cube::new();
            let ray = Ray::new(origin, direction);
            assert!(cube.intersect(&ray).is_empty());
        }
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let cases = vec![
            (Point::at(1.0, 0.5, -0.8) , Vector::new(1, 0, 0)) ,
            (Point::at(-1.0, -0.2, 0.9), Vector::new(-1, 0, 0)),
            (Point::at(-0.4, 1.0, -0.1), Vector::new(0, 1, 0)),
            (Point::at(0.3, -1.0, -0.7), Vector::new(0, -1, 0)),
            (Point::at(-0.6, 0.3, 1.0), Vector::new(0, 0, 1)),
            (Point::at(0.4, 0.4, -1.0), Vector::new(0, 0, -1)),
            (Point::at(1, 1, 1), Vector::new(1, 0, 0)),
            (Point::at(-1, -1, -1), Vector::new(-1, 0, 0)),
        ];

        for (point, expected_normal) in cases {
            let cube = Cube::new();
            assert_eq!(expected_normal, cube.normal_at(point));
        }
    }
}
