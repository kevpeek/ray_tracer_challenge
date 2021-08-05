use crate::geometry::matrix::Matrix;
use crate::tracing::ray::Ray;
use crate::tracing::intersection::Intersections;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Intersections;
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

    struct TestShape {
        expected_ray: Ray
    }

    impl Shape for TestShape {
        fn intersect(&self, ray: &Ray) -> Intersections {
            assert_eq!(*ray, self.expected_ray);
            intersections!()
        }
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let expected_ray = Ray::new(Point::at(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5));

        let transform = transformations::scaling(2, 2, 2);
        let shape = TestShape { expected_ray };
        let transformed_shape = TransformedShape::new(Box::new(shape), transform);

        transformed_shape.intersect(&ray);
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let expected_ray = Ray::new(Point::at(-5, 0, -5), Vector::new(0, 0, 1));

        let shape = TestShape { expected_ray };
        let transform = transformations::translation(5, 0, 0);
        let transformed_shape = TransformedShape::new(Box::new(shape), transform);

        transformed_shape.intersect(&ray);
    }
}
