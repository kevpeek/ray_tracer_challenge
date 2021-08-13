use crate::geometry::matrix::Matrix;
use crate::tracing::ray::Ray;
use crate::tracing::intersection::{Intersections, Intersection};
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use std::fmt::Debug;
use crate::tracing::material::Material;
use std::any::Any;
use crate::tracing::sphere::Sphere;

pub type WorldShape = Box<dyn Shape>;

impl Clone for WorldShape {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for WorldShape {
    fn eq(&self, other: &WorldShape) -> bool {
        self.box_eq(other.as_any())
    }
}

pub trait Shape: Any + Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> WorldShape;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn material(&self) -> &Material;
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal_at(&self, point: Point) -> Vector;
}


#[derive(Clone, Debug)]
pub struct TransformedShape {
    transformation: Matrix,
    delegate: WorldShape
}

impl PartialEq for TransformedShape {
    fn eq(&self, other: &TransformedShape) -> bool {
        self.delegate.box_eq(&other.delegate) && self.transformation == other.transformation
    }
}

impl TransformedShape {
    pub fn new(delegate: WorldShape, transformation: Matrix) -> TransformedShape {
        TransformedShape {
            transformation,
            delegate
        }
    }
}

impl Shape for TransformedShape {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> WorldShape {
        Box::new((*self).clone())
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
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
        let corrected_intersections = delegate_intersections.intersections.iter()
            .map(Intersection::time)
            .map(|time| Intersection::new(time, self.box_clone()))
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

#[cfg(test)]
mod tests {
    use crate::tracing::ray::Ray;
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;
    use crate::geometry::transformations;
    use crate::tracing::shape::{Shape, TransformedShape, WorldShape};
    use crate::tracing::intersection::Intersections;
    use crate::intersections;
    use crate::tracing::sphere::Sphere;
    use std::f64::consts::PI;
    use crate::tracing::material::Material;
    use std::any::Any;

    #[derive(Debug, Clone, PartialEq)]
    struct TestShape {
        material: Material,
        expected_ray: Option<Ray>,
    }

    impl TestShape {
        fn new() -> TestShape {
            TestShape { expected_ray: None, material: Material::default() }
        }
        fn with_ray(ray: Ray) -> TestShape {
            TestShape {
                expected_ray: Some(ray),
                material: Material::default()
            }
        }
    }

    impl Shape for TestShape {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn box_clone(&self) -> WorldShape {
            Box::new((*self).clone())
        }

        fn box_eq(&self, other: &dyn Any) -> bool {
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
        let rotation = transformations::rotation_z(PI/5.0);
        let transform = &scaling * &rotation;
        let transformed_shape = TransformedShape::new(Box::new(shape), transform);
        let actual_normal = transformed_shape.normal_at(Point::at(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0));
        let expected_normal = Vector::new(0.0, 0.97014, -0.24254);
        assert_eq!(expected_normal, actual_normal);
    }
}
