use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::tracing::intersection::{Intersection, Intersections};
use crate::tracing::material::Material;
use crate::tracing::ray::Ray;
use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;

pub type WorldShape<'a> = &'a dyn Shape;

impl<'a> PartialEq for WorldShape<'a> {
    fn eq(&self, other: &WorldShape) -> bool {
        self.equals_shape(other.as_any())
    }
}

pub trait Shape: ShapeClone + Any + Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
    fn equals_shape(&self, other: &dyn Any) -> bool;
    fn material(&self) -> &Material;
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal_at(&self, point: Point) -> Vector;
}

#[derive(Debug, Clone)]
pub struct TransformedShape {
    transformation: Matrix,
    delegate: Box<dyn Shape>,
}

impl PartialEq for TransformedShape {
    fn eq(&self, other: &TransformedShape) -> bool {
        self.delegate.equals_shape(&other.delegate) && self.transformation == other.transformation
    }
}

impl TransformedShape {
    pub fn new(delegate: Box<dyn Shape>, transformation: Matrix) -> TransformedShape {
        TransformedShape {
            transformation,
            delegate,
        }
    }
}

impl Shape for TransformedShape {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals_shape(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn material(&self) -> &Material {
        self.delegate.material()
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(self.transformation.inverse());

        let delegate_intersections = self.delegate.intersect(&local_ray);

        // The delegate will return intersections that contain copies of delegate, not wrapped by this struct.
        // Recreate those intersections with the same times, but using this shape.
        let corrected_intersections = delegate_intersections
            .intersections
            .iter()
            .map(Intersection::time)
            .map(|time| Intersection::new(time, self))
            .collect();

        Intersections::new(corrected_intersections)
    }

    fn normal_at(&self, point: Point) -> Vector {
        let local_point = &self.transformation.inverse() * point;
        let local_normal = self.delegate.normal_at(local_point);
        let world_normal = &self.transformation.inverse().transpose() * local_normal;
        world_normal.normalize()
    }
}

pub trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T> ShapeClone for T
    where
        T: 'static + Shape + Clone,
{
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_box()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::vector::Vector;
    use crate::intersections;
    use crate::tracing::intersection::Intersections;
    use crate::tracing::material::Material;
    use crate::tracing::ray::Ray;
    use crate::tracing::shape::{Shape, TransformedShape, WorldShape};
    use crate::tracing::sphere::Sphere;
    use std::any::Any;
    use std::f64::consts::PI;

    #[derive(Debug, Clone, PartialEq)]
    struct TestShape {
        material: Material,
        expected_ray: Option<Ray>,
    }

    impl TestShape {
        fn new() -> TestShape {
            TestShape {
                expected_ray: None,
                material: Material::default(),
            }
        }
        fn with_ray(ray: Ray) -> TestShape {
            TestShape {
                expected_ray: Some(ray),
                material: Material::default(),
            }
        }
    }

    impl Shape for TestShape {
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
            assert_eq!(*ray, *self.expected_ray.as_ref().unwrap());
            intersections!()
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
        let shape = TestShape::with_ray(expected_ray);
        let transformed_shape = TransformedShape::new(Box::new(shape), transform);

        transformed_shape.intersect(&ray);
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let expected_ray = Ray::new(Point::at(-5, 0, -5), Vector::new(0, 0, 1));

        let shape = TestShape::with_ray(expected_ray);
        let transform = transformations::translation(5, 0, 0);
        let transformed_shape = TransformedShape::new(Box::new(shape), transform);

        transformed_shape.intersect(&ray);
    }

    #[test]
    fn compute_normal_on_translated_shape() {
        let shape = TestShape::new();
        let transform = transformations::translation(0, 1, 0);

        let transformed_shape = TransformedShape::new(Box::new(shape), transform);

        let expected_normal = Vector::new(0.0, 0.70711, -0.70711);
        let actual_normal = transformed_shape.normal_at(Point::at(0.0, 1.70711, -0.70711));
        assert_eq!(expected_normal, actual_normal);
    }

    #[test]
    fn computing_normal_of_transformed_shape() {
        let shape = TestShape::new();
        let scaling = transformations::scaling(1.0, 0.5, 1.0);
        let rotation = transformations::rotation_z(PI / 5.0);
        let transform = &scaling * &rotation;
        let transformed_shape = TransformedShape::new(Box::new(shape), transform);
        let actual_normal = transformed_shape.normal_at(Point::at(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));
        let expected_normal = Vector::new(0.0, 0.97014, -0.24254);
        assert_eq!(expected_normal, actual_normal);
    }
}
