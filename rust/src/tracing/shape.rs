use crate::geometry::matrix::Matrix;
use crate::tracing::ray::Ray;
use crate::tracing::intersection::Intersections;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal_at(&self, point: Point) -> Vector;
}

pub struct TransformedShape {
    transformation: Matrix,
    delegate: Box<dyn Shape>
}

impl TransformedShape {
    pub fn new(delegate: Box<dyn Shape>, transformation: Matrix) -> TransformedShape {
        TransformedShape {
            transformation,
            delegate
        }
    }
}

impl Shape for TransformedShape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(self.transformation.inverse());
        self.delegate.intersect(&local_ray)
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
    use crate::tracing::shape::{Shape, TransformedShape};
    use crate::tracing::intersection::Intersections;
    use crate::intersections;
    use crate::tracing::sphere::Sphere;
    use std::f64::consts::PI;

    struct TestShape {
        expected_ray: Option<Ray>,
    }

    impl TestShape {
        fn new() -> TestShape {
            TestShape { expected_ray: None }
        }
        fn with_ray(ray: Ray) -> TestShape {
            TestShape {
                expected_ray: Some(ray),
            }
        }
    }

    impl Shape for TestShape {
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
