use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::intersections;
use crate::tracing::intersection::{Intersection, Intersections};
use crate::tracing::material::Material;
use crate::tracing::ray::Ray;
use crate::tracing::shape::{Shape, TransformedShape};
use std::any::Any;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Sphere {
    origin: Point,
    material: Material,
}

impl Shape for Sphere {
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
        let sphere_to_ray = ray.origin() - self.origin;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return intersections![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        intersections![Intersection::new(t1, self), Intersection::new(t2, self)]
    }

    /**
     * Return the Vector normal to this sphere at the supplied point.
     */
    fn normal_at(&self, point: Point) -> Vector {
        point - self.origin
    }
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere {
            origin: Point::origin(),
            material: Material::default(),
        }
    }

    pub fn new(origin: Point, material: Material) -> Sphere {
        Sphere { origin, material }
    }

    pub fn with_origin(self, new_origin: Point) -> Sphere {
        Sphere::new(new_origin, self.material)
    }

    pub fn with_material(self, new_material: Material) -> Sphere {
        Sphere::new(self.origin, new_material)
    }

    pub fn with_transform(self, new_transform: Matrix) -> TransformedShape {
        TransformedShape::new(Box::new(self), new_transform)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{rotation_z, scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::tracing::material::Material;
    use crate::tracing::shape::Shape;
    use crate::tracing::sphere::Sphere;
    use std::f64::consts::PI;

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::at(1, 0, 0));

        assert_eq!(Vector::new(1, 0, 0), normal);
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::at(0, 1, 0));

        assert_eq!(Vector::new(0, 1, 0), normal);
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::at(0, 0, 1));

        assert_eq!(Vector::new(0, 0, 1), normal);
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::at(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            Vector::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            ),
            normal
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::at(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(normal.normalize(), normal);
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let sphere = Sphere::default().with_transform(translation(0, 1, 0));

        let normal = sphere.normal_at(Point::at(0.0, 1.70711, -0.70711));
        assert_eq!(Vector::new(0.0, 0.70711, -0.70711), normal);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let sphere =
            Sphere::default().with_transform(&scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0));

        let normal = sphere.normal_at(Point::at(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        assert_eq!(Vector::new(0.0, 0.97014, -0.24254), normal);
    }

    #[test]
    fn sphere_has_default_material() {
        let sphere = Sphere::default();

        assert_eq!(Material::default(), sphere.material);
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let material = Material::default();
        let mut sphere = Sphere::default();
        sphere.material = material.clone();

        assert_eq!(material, sphere.material);
    }
}
