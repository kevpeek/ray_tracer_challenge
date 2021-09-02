use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::EPSILON;
use crate::intersections;
use crate::tracing::intersection::{Intersection, Intersections};
use crate::tracing::material::Material;
use crate::tracing::ray::Ray;
use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
use std::any::Any;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }
}

impl ShapeGeometry for Plane {
    fn name(&self) -> &'static str {
        "plane"
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        if ray.direction().y.abs() < EPSILON {
            return vec![];
        }

        let time = -ray.origin().y / ray.direction().y;
        vec![time]
    }

    fn normal_at(&self, point: Point) -> Vector {
        Vector::new(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;
    use crate::intersections;
    use crate::tracing::intersection::Intersection;
    use crate::tracing::shapes::plane::Plane;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::shape::ShapeGeometry;

    #[test]
    fn normal_of_plane_is_constant() {
        let plane = Plane::new();
        assert_eq!(Vector::new(0, 1, 0), plane.normal_at(Point::at(0, 0, 0)));
        assert_eq!(Vector::new(0, 1, 0), plane.normal_at(Point::at(10, 0, -10)));
        assert_eq!(Vector::new(0, 1, 0), plane.normal_at(Point::at(-5, 0, 150)));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let plane = Plane::new();
        let ray = Ray::new(Point::at(0, 10, 0), Vector::new(0, 0, 1));
        assert!(plane.intersect(&ray).is_empty())
    }

    #[test]
    fn intersect_with_coplaner_ray() {
        let plane = Plane::new();
        let ray = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        assert!(plane.intersect(&ray).is_empty())
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let plane = Plane::new();
        let ray = Ray::new(Point::at(0, 1, 0), Vector::new(0, -1, 0));
        let intersections = plane.intersect(&ray);

        let expected = vec![1.0];
        assert_eq!(expected, intersections);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let plane = Plane::new();
        let ray = Ray::new(Point::at(0, -1, 0), Vector::new(0, 1, 0));
        let intersections = plane.intersect(&ray);

        let expected = vec![1.0];
        assert_eq!(expected, intersections);
    }
}
