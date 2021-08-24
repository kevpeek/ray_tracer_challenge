use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::EPSILON;
use crate::intersections;
use crate::tracing::intersection::{Intersection, Intersections};
use crate::tracing::material::Material;
use crate::tracing::ray::Ray;
use crate::tracing::shape::{Shape, TransformedShape};
use std::any::Any;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Plane {
    material: Material,
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            material: Material::default(),
        }
    }

    pub fn with_material(self, new_material: Material) -> Plane {
        Plane {
            material: new_material,
        }
    }

    pub fn with_transform(self, new_transform: Matrix) -> TransformedShape {
        TransformedShape::new(Box::new(self), new_transform)
    }
}

impl Shape for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals_shape(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        if ray.direction().y.abs() < EPSILON {
            return Intersections::empty();
        }

        let time = -ray.origin().y / ray.direction().y;
        intersections![Intersection::new(time, self)]
    }

    fn normal_at(&self, _: Point) -> Vector {
        Vector::new(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;
    use crate::intersections;
    use crate::tracing::intersection::Intersection;
    use crate::tracing::plane::Plane;
    use crate::tracing::ray::Ray;
    use crate::tracing::shape::Shape;

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

        let expected = intersections![Intersection::new(1.0, &plane)];
        assert_eq!(expected, intersections);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let plane = Plane::new();
        let ray = Ray::new(Point::at(0, -1, 0), Vector::new(0, 1, 0));
        let intersections = plane.intersect(&ray);

        let expected = intersections![Intersection::new(1.0, &plane)];
        assert_eq!(expected, intersections);
    }
}
