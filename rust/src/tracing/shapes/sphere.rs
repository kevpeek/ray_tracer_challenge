use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::tracing::ray::Ray;
use crate::tracing::shapes::shape::ShapeGeometry;

/**
 * Sphere represents a unit sphere centered at the origin.
*/
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }

    fn origin(&self) -> Point {
        Point::origin()
    }
}

impl ShapeGeometry for Sphere {
    fn name(&self) -> &'static str {
        "sphere"
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin() - self.origin();
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![t1, t2]
    }

    /**
     * Return the Vector normal to this sphere at the supplied point.
     */
    fn normal_at(&self, point: Point) -> Vector {
        point - self.origin()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{rotation_z, scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::tracing::material::Material;
    use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
    use crate::tracing::shapes::sphere::Sphere;
    use std::f64::consts::PI;

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let sphere = Shape::sphere();
        let normal = sphere.normal_at(Point::at(1, 0, 0));

        assert_eq!(Vector::new(1, 0, 0), normal);
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let sphere = Shape::sphere();
        let normal = sphere.normal_at(Point::at(0, 1, 0));

        assert_eq!(Vector::new(0, 1, 0), normal);
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let sphere = Shape::sphere();
        let normal = sphere.normal_at(Point::at(0, 0, 1));

        assert_eq!(Vector::new(0, 0, 1), normal);
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let sphere = Shape::sphere();
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
        let sphere = Shape::sphere();
        let normal = sphere.normal_at(Point::at(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(normal.normalize(), normal);
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let sphere = Shape::sphere().with_transform(translation(0, 1, 0));

        let normal = sphere.normal_at(Point::at(0.0, 1.70711, -0.70711));
        assert_eq!(Vector::new(0.0, 0.70711, -0.70711), normal);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let sphere =
            Shape::sphere().with_transform(&scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0));

        let normal = sphere.normal_at(Point::at(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        assert_eq!(Vector::new(0.0, 0.97014, -0.24254), normal);
    }
}
