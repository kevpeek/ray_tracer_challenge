use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::tracing::material::Material;
use crate::tracing::intersection::{Intersections, Intersection};
use crate::tracing::ray::Ray;
use crate::intersections;
use crate::tracing::shape::Shape;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Sphere {
    origin: Point,
    material: Material,
    transform: Matrix
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(self.transform().inverse());
        let sphere_to_ray = transformed_ray.origin() - self.origin();
        let a = transformed_ray.direction().dot(transformed_ray.direction());
        let b = 2.0 * transformed_ray.direction().dot(sphere_to_ray);
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
        let transform_to_object_space = self.transform.inverse();
        let point_in_object_space = &transform_to_object_space * point;
        let normal_in_object_space = point_in_object_space - self.origin;
        let transform_to_world_space = self.transform.submatrix(3, 3).inverse().transpose();
        (&transform_to_world_space * normal_in_object_space).normalize()
    }
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere {
            origin: Point::origin(),
            material: Material::default(),
            transform: Matrix::identity(4)
        }
    }

    pub fn new(origin: Point, material: Material, transform: Matrix) -> Sphere {
        Sphere {
            origin,
            material,
            transform
        }
    }

    pub fn with_origin(self, new_origin: Point) -> Sphere {
        Sphere::new(new_origin, self.material, self.transform)
    }

    pub fn with_material(self, new_material: Material) -> Sphere {
        Sphere::new(self.origin, new_material, self.transform)
    }

    pub fn with_transform(self, new_transform: Matrix) -> Sphere {
        Sphere::new(self.origin, self.material, new_transform)
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{rotation_z, scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::tracing::material::Material;
    use crate::tracing::sphere::Sphere;
    use std::f64::consts::PI;
    use crate::tracing::shape::Shape;

    #[test]
    fn spheres_default_transformation() {
        let sphere = Sphere::default();
        assert_eq!(Matrix::identity(4), sphere.transform);
    }

    #[test]
    fn changing_spheres_transformation() {
        let transformation = translation(2, 3, 4);
        let mut sphere = Sphere::default();
        sphere.transform = transformation.clone();
        assert_eq!(transformation, sphere.transform);
    }

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
        let mut sphere = Sphere::default();
        sphere.transform = translation(0, 1, 0);

        let normal = sphere.normal_at(Point::at(0.0, 1.70711, -0.70711));
        assert_eq!(Vector::new(0.0, 0.70711, -0.70711), normal);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut sphere = Sphere::default();
        sphere.transform = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);

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
