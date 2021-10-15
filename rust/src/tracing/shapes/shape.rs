use std::any::Any;
use std::fmt::Debug;

use crate::display::color::Color;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::tracing::intersection::{Intersection, Intersections};
use crate::tracing::material::Material;
use crate::tracing::point_light::PointLight;
use crate::tracing::ray::Ray;
use crate::tracing::shapes::plane::Plane;
use crate::tracing::shapes::sphere::Sphere;

pub type WorldShape<'a> = &'a Shape;

///ShapeGeometry is a Strategy defining the geometric formulas of a shape.
pub trait ShapeGeometry: GeometryClone + Any + Send + Sync + Debug {
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, point: Point) -> Vector;
    fn into_shape(self) -> Shape
    where
        Self: Sized,
    {
        self.into()
    }
}

impl<T: ShapeGeometry> From<T> for Shape {
    fn from(geometry: T) -> Self {
        Shape::using(geometry)
    }
}

/// A shape in the world. Combines the geometry with material and transformation.
#[derive(Debug, Clone)]
pub struct Shape {
    geometry: Box<dyn ShapeGeometry>,
    material: Material,
    transformation: Matrix,

    // For performing computations, we really need the inverse and its transpose.
    // Because we would calculate these multiple times per ray, let's precompute them.
    transform_inverse: Matrix,
    transform_inverse_transpose: Matrix,
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
            transform_inverse: Matrix::identity(4),
            transform_inverse_transpose: Matrix::identity(4),
        }
    }

    pub fn with_material(self, material: Material) -> Shape {
        Shape {
            material,
            ..self
        }
    }

    pub fn with_transform(self, transformation: Matrix) -> Shape {
        let transform_inverse = transformation.inverse();
        let transform_inverse_transpose = transform_inverse.transpose();
        Shape {
            transformation,
            transform_inverse,
            transform_inverse_transpose,
            ..self
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    /// Calculate when the supplied Ray intersects this shape.
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(self.transform_inverse.clone());

        let intersection_times = self.geometry.intersect(&local_ray);
        let intersections = intersection_times
            .into_iter()
            .map(|time| Intersection::new(time, self))
            .collect();

        Intersections::new(intersections)
    }

    /// The normal vector of this shape at the point provided.
    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = &self.transform_inverse * point;
        let local_normal = self.geometry.normal_at(local_point);
        let world_normal = &self.transform_inverse_transpose * local_normal;
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
        let transformed_point = &self.transform_inverse * position;
        self.material()
            .lighting(light, transformed_point, eye_vector, normal, in_shadow)
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Shape) -> bool {
        // Originally written for tests but now used in non-test code (see Intersection::find_refractive_indexes).
        // Perhaps not the right way to do this since two separate instances could resolve to equal.
        // Nevertheless, two identical instances would be pointless in a World, so maybe safe to ignore.
        compare_geometries(self.geometry.as_ref(), other.geometry.as_ref())
            && self.transformation == other.transformation
            && self.material == other.material
    }
}

/// Determine if two ShapeGeometry instances are equal.
fn compare_geometries(one: &dyn ShapeGeometry, two: &dyn ShapeGeometry) -> bool {
    // Quick and dirty way to evaluate equality for two geometry instances
    format!("{:?}", one) == format!("{:?}", two)
}

pub trait GeometryClone {
    fn clone_box(&self) -> Box<dyn ShapeGeometry>;
}

impl<T> GeometryClone for T
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
