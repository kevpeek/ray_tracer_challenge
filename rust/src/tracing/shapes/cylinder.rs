use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helpers::approximate;
use crate::helpers::approximate::Approximate;
use crate::helpers::general::OrderedTuple;
use crate::tracing::ray::Ray;
use crate::tracing::shapes::shape::ShapeGeometry;
use num::traits::real::Real;
use num::traits::Pow;

#[derive(PartialEq, Debug, Clone)]
pub struct Cylinder {
    min: f64,
    max: f64,
    capped: bool,
}

impl Cylinder {
    pub fn new(min: f64, max: f64) -> Cylinder {
        Cylinder {
            min,
            max,
            capped: false,
        }
    }

    pub fn infinite() -> Cylinder {
        Cylinder::new(f64::NEG_INFINITY, f64::INFINITY)
    }

    pub fn capped(self) -> Cylinder {
        Cylinder {
            min: self.min,
            max: self.max,
            capped: true,
        }
    }

    // Find intersections with cylinder end caps.
    fn intersect_caps(&self, ray: &Ray) -> Vec<f64> {
        if !self.capped {
            return Vec::new();
        }

        let value = ray.direction().y;
        if value.almost_zero() {
            return Vec::new();
        }

        vec![self.min, self.max]
            .into_iter()
            .map(|y_value| (y_value - ray.origin().y) / ray.direction().y)
            .filter(|time| check_cap(ray, *time))
            .collect()
    }
}

// Determine if the Ray would be within the cylinder's radius at the time supplied.
fn check_cap(ray: &Ray, time: f64) -> bool {
    let x = ray.origin().x + time * ray.direction().x;
    let z = ray.origin().z + time * ray.direction().z;

    x.pow(2) + z.pow(2) <= 1.0
}

impl ShapeGeometry for Cylinder {
    fn name(&self) -> &'static str {
        "cylinder"
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let a: f64 = (ray.direction().x.pow(2) + ray.direction().z.pow(2));
        if a.almost_zero() {
            return self.intersect_caps(ray);
        }

        let b = 2.0 * ray.direction().x * ray.origin().x + 2.0 * ray.direction().z * ray.origin().z;
        let c = ray.origin().x.pow(2) + ray.origin().z.pow(2) - 1.0;

        let discriminant: f64 =
            b.pow(2) - 4.0 * (ray.direction().x.pow(2) + ray.direction().z.pow(2)) * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t0: f64 = (-b - discriminant.sqrt())
            / (2.0 * (ray.direction().x.pow(2) + ray.direction().z.pow(2)));
        let t1: f64 = (-b + discriminant.sqrt())
            / (2.0 * (ray.direction().x.pow(2) + ray.direction().z.pow(2)));

        // Ensure t0 and t1 are ordered
        let (t0, t1) = (t0, t1).ordered();

        let intersections: Vec<f64> = vec![t0, t1]
            .into_iter()
            .map(|time| (time, ray.origin().y + time * ray.direction().y))
            .filter(|(time, y_value)| self.min < *y_value && *y_value < self.max)
            .map(|(time, _)| time)
            .collect();

        let mut cap_intersections = self.intersect_caps(ray);
        cap_intersections.extend_from_slice(&intersections);
        cap_intersections
    }

    fn normal_at(&self, point: Point) -> Vector {
        let distance = point.x.pow(2) + point.z.pow(2);

        if distance < 1.0 && point.y >= self.max - approximate::EPSILON {
            return Vector::new(0, 1, 0);
        }

        if distance < 1.0 && point.y <= self.min + approximate::EPSILON {
            return Vector::new(0, -1, 0);
        }

        Vector::new(point.x, 0.0, point.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;
    use crate::helpers::approximate::Approximate;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::cylinder::Cylinder;
    use crate::tracing::shapes::shape::ShapeGeometry;

    #[test]
    fn ray_misses_a_cylinder() {
        let cases = vec![
            (Point::at(1, 0, 0), Vector::new(0, 1, 0)),
            (Point::at(0, 0, 0), Vector::new(0, 1, 0)),
            (Point::at(0, 0, -5), Vector::new(1, 1, 1)),
        ];

        for (origin, direction) in cases {
            let cylinder = Cylinder::infinite();
            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);
            assert!(cylinder.intersect(&ray).is_empty());
        }
    }

    #[test]
    fn ray_strikes_a_cylinder() {
        let cases = vec![
            (Point::at(1, 0, -5), Vector::new(0, 0, 1), 5.0, 5.0),
            (Point::at(0, 0, -5), Vector::new(0, 0, 1), 4.0, 6.0),
            (
                Point::at(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        for (origin, direction, t1, t2) in cases {
            let cylinder = Cylinder::infinite();
            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);
            let intersections = cylinder.intersect(&ray);
            assert!(intersections[0].almost(t1));
            assert!(intersections[1].almost(t2));
        }
    }

    #[test]
    fn normal_vector_of_cylinder() {
        let cases = vec![
            (Point::at(1, 0, 0), Vector::new(1, 0, 0)),
            (Point::at(0, 5, -1), Vector::new(0, 0, -1)),
            (Point::at(0, -2, 1), Vector::new(0, 0, 1)),
            (Point::at(-1, 1, 0), Vector::new(-1, 0, 0)),
        ];

        for (point, normal) in cases {
            let cylinder = Cylinder::infinite();
            assert_eq!(normal, cylinder.normal_at(point));
        }
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let cases = vec![
            (Point::at(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::at(0, 3, -5), Vector::new(0, 0, 1), 0),
            (Point::at(0, 0, -5), Vector::new(0, 0, 1), 0),
            (Point::at(0, 2, -5), Vector::new(0, 0, 1), 0),
            (Point::at(0, 1, -5), Vector::new(0, 0, 1), 0),
            (Point::at(0.0, 1.5, -2.0), Vector::new(0, 0, 1), 2),
        ];

        for (origin, direction, count) in cases {
            let cylinder = Cylinder::new(1.0, 2.0);
            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);
            assert_eq!(count, cylinder.intersect(&ray).len());
        }
    }

    #[test]
    fn intersect_caps_of_closed_cylinder() {
        let cases = vec![
            (Point::at(0, 3, 0), Vector::new(0, -1, 0), 2),
            (Point::at(0, 3, -2), Vector::new(0, -1, 2), 2),
            (Point::at(0, 4, -2), Vector::new(0, -1, 1), 2),
            (Point::at(0, 0, -2), Vector::new(0, 1, 2), 2),
            (Point::at(0, -1, -2), Vector::new(0, 1, 1), 2),
        ];

        for (point, direction, count) in cases {
            let cylinder = Cylinder::new(1.0, 2.0).capped();
            let direction = direction.normalize();
            let ray = Ray::new(point, direction);
            assert_eq!(count, cylinder.intersect(&ray).len());
        }
    }

    #[test]
    fn normal_at_end_caps() {
        let cases = vec![
            (Point::at(0, 1, 0), Vector::new(0, -1, 0)),
            (Point::at(0.5, 1.0, 0.0), Vector::new(0, -1, 0)),
            (Point::at(0.0, 1.0, 0.5), Vector::new(0, -1, 0)),
            (Point::at(0, 2, 0), Vector::new(0, 1, 0)),
            (Point::at(0.5, 2.0, 0.0), Vector::new(0, 1, 0)),
            (Point::at(0.0, 2.0, 0.5), Vector::new(0, 1, 0)),
        ];

        for (point, normal) in cases {
            let cylinder = Cylinder::new(1.0, 2.0).capped();
            assert_eq!(normal, cylinder.normal_at(point));
        }
    }
}
