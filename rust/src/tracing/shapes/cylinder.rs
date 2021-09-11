use crate::tracing::shapes::shape::ShapeGeometry;
use crate::tracing::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use num::traits::Pow;
use crate::helper::almost;
use num::traits::real::Real;

#[derive(PartialEq, Debug, Clone)]
pub struct Cylinder {
    min: f64,
    max: f64,
}

impl Cylinder {
    pub fn new(min: f64, max: f64) -> Cylinder {
        Cylinder { min, max }
    }

    pub fn infinite() -> Cylinder {
        Cylinder::new(f64::NEG_INFINITY, f64::INFINITY)
    }
}

impl ShapeGeometry for Cylinder {
    fn name(&self) -> &'static str {
        "cylinder"
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let a = ray.direction().x.pow(2) + ray.direction().z.pow(2);
        if almost(a, 0.0) {
            return Vec::new();
        }

        let b = 2.0 * ray.direction().x * ray.origin().x + 2.0 * ray.direction().z * ray.origin().z;
        let c = ray.origin().x.pow(2) + ray.origin().z.pow(2) - 1.0;

        let discriminant: f64 = b.pow(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let mut intersections = Vec::new();

        let t_min = t0.min(t1);

        let y_min = ray.origin().y + t_min * ray.direction().y;
        if self.min < y_min && y_min < self.max {
            intersections.push(t_min);
        }

        let t_max = t0.max(t1);
        let y_max = ray.origin().y + t_max * ray.direction().y;
        if self.min < y_max && y_max < self.max {
            intersections.push(t_max);
        }


        intersections
    }

    fn normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, 0.0, point.z)
    }
}


#[cfg(test)]
mod tests {
    use crate::tracing::shapes::cylinder::Cylinder;
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::shape::ShapeGeometry;
    use crate::helper::almost;

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
            (Point::at(0.5, 0.0, -5.0), Vector::new(0.1, 1.0, 1.0), 6.80798, 7.08872),
        ];

        for (origin, direction, t1, t2) in cases {
            let cylinder = Cylinder::infinite();
            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);
            let intersections = cylinder.intersect(&ray);
            assert!(almost(t1, intersections[0]));
            assert!(almost(t2, intersections[1]));
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
            (Point::at(0, 3, -5), Vector::new(0, 0, 1)   , 0),
            (Point::at(0, 0, -5) , Vector::new(0, 0, 1), 0),
            (Point::at(0, 2, -5) , Vector::new(0, 0, 1), 0),
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
}
