use std::any::Any;
use std::fmt::Debug;

use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::tracing::intersection::{Intersection, Intersections};
use crate::tracing::material::Material;
use crate::tracing::ray::Ray;
use crate::display::color::Color;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::sphere::Sphere;
use crate::tracing::shapes::plane::Plane;

pub type WorldShape<'a> = &'a Shape;

/**
 * ShapeGeometry is a Strategy defining the geometric formulas of a shape.
*/
pub trait ShapeGeometry: ShapeClone + Any + Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, point: Point) -> Vector;
    fn into_shape(self) -> Shape where Self: Sized {
        Shape::using(self)
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    geometry: Box<dyn ShapeGeometry>,
    material: Material,
    transformation: Matrix,
}


impl Shape {
    pub fn sphere() -> Shape {
        Sphere::new().into_shape()
    }

    pub fn plane() -> Shape {
        Plane::new().into_shape()
    }

    pub fn using<T: ShapeGeometry>(geometry: T) -> Shape {
        Shape {
            geometry: Box::new(geometry),
            material: Material::default(),
            transformation: Matrix::identity(4),
        }
    }

    pub fn with_material(self, material: Material) -> Shape {
        Shape {
            geometry: self.geometry,
            material,
            transformation: self.transformation,
        }
    }

    pub fn with_transform(self, transformation: Matrix) -> Shape {
        Shape {
            geometry: self.geometry,
            material: self.material,
            transformation,
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(self.transformation.inverse());

        let delegate_intersections = self.geometry.intersect(&local_ray);

        let corrected_intersections = delegate_intersections
            .into_iter()
            .map(|time| Intersection::new(time, self))
            .collect();

        Intersections::new(corrected_intersections)
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = &self.transformation.inverse() * point;
        let local_normal = self.geometry.normal_at(local_point);
        let world_normal = &self.transformation.inverse().transpose() * local_normal;
        world_normal.normalize()
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: Point,
        eye_vector: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        let transformed_point = &self.transformation.inverse() * position;
        self.material().lighting(light, transformed_point, eye_vector, normal, in_shadow)
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Shape) -> bool {
        self.geometry.name() == other.geometry.name()
            && self.transformation == other.transformation
            && self.material == other.material
    }
}

pub trait ShapeClone {
    fn clone_box(&self) -> Box<dyn ShapeGeometry>;
}

impl<T> ShapeClone for T
where
    T: 'static + ShapeGeometry + Clone,
{
    fn clone_box(&self) -> Box<dyn ShapeGeometry> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn ShapeGeometry> {
    fn clone(&self) -> Box<dyn ShapeGeometry> {
        self.clone_box()
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::f64::consts::PI;

    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::vector::Vector;
    use crate::intersections;
    use crate::tracing::intersection::Intersections;
    use crate::tracing::material::Material;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::shape::{Shape, ShapeGeometry};

    #[derive(Debug, Clone, PartialEq)]
    struct TestGeometry {
        material: Material,
        expected_ray: Option<Ray>,
    }

    impl TestGeometry {
        fn new() -> TestGeometry {
            TestGeometry {
                expected_ray: None,
                material: Material::default(),
            }
        }
        fn with_ray(ray: Ray) -> TestGeometry {
            TestGeometry {
                expected_ray: Some(ray),
                material: Material::default(),
            }
        }
    }

    impl ShapeGeometry for TestGeometry {
        fn name(&self) -> &'static str {
            "test_shape"
        }

        fn intersect(&self, ray: &Ray) -> Vec<f64> {
            assert_eq!(*ray, *self.expected_ray.as_ref().unwrap());
            vec![]
        }

        fn normal_at(&self, point: Point) -> Vector {
            Vector::new(point.x, point.y, point.z)
        }
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let expected_ray = Ray::new(Point::at(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5));

        let transform = transformations::scaling(2, 2, 2);
        let geometry = TestGeometry::with_ray(expected_ray);
        let transformed_shape = geometry.into_shape().with_transform(transform);

        transformed_shape.intersect(&ray);
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let expected_ray = Ray::new(Point::at(-5, 0, -5), Vector::new(0, 0, 1));

        let geometry = TestGeometry::with_ray(expected_ray);
        let transform = transformations::translation(5, 0, 0);
        let transformed_shape = geometry.into_shape().with_transform(transform);

        transformed_shape.intersect(&ray);
    }

    #[test]
    fn compute_normal_on_translated_shape() {
        let geometry = TestGeometry::new();
        let transform = transformations::translation(0, 1, 0);

        let transformed_shape = geometry.into_shape().with_transform(transform);

        let expected_normal = Vector::new(0.0, 0.70711, -0.70711);
        let actual_normal = transformed_shape.normal_at(Point::at(0.0, 1.70711, -0.70711));
        assert_eq!(expected_normal, actual_normal);
    }

    #[test]
    fn computing_normal_of_transformed_shape() {
        let geometry = TestGeometry::new();
        let scaling = transformations::scaling(1.0, 0.5, 1.0);
        let rotation = transformations::rotation_z(PI / 5.0);
        let transform = &scaling * &rotation;
        let transformed_shape = geometry.into_shape().with_transform(transform);
        let actual_normal = transformed_shape.normal_at(Point::at(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));
        let expected_normal = Vector::new(0.0, 0.97014, -0.24254);
        assert_eq!(expected_normal, actual_normal);
    }
}
