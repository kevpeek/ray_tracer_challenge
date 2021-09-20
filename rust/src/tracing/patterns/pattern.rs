use crate::display::color::Color;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use std::any::Any;
use std::fmt::Debug;

pub type PatternType = TransformedPattern;

impl PartialEq for PatternType {
    fn eq(&self, other: &PatternType) -> bool {
        &self.delegate == &other.delegate && self.transform == other.transform
    }
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Self) -> bool {
        self.equals_pattern(other.as_any())
    }
}

pub trait Pattern: PatternClone + Any + Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
    fn equals_pattern(&self, other: &dyn Any) -> bool;
    fn pattern_at(&self, point: Point) -> Color;
}

#[derive(Debug, Clone)]
pub struct TransformedPattern {
    delegate: Box<dyn Pattern>,
    transform: Matrix,
}

pub trait PatternClone {
    fn clone_box(&self) -> Box<dyn Pattern>;
}

impl<T> PatternClone for T
where
    T: 'static + Pattern + Clone,
{
    fn clone_box(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Box<dyn Pattern> {
        self.clone_box()
    }
}

impl TransformedPattern {
    pub fn new(delegate: Box<dyn Pattern>, transform: Matrix) -> TransformedPattern {
        TransformedPattern {
            delegate,
            transform,
        }
    }

    pub fn using_identity(delegate: Box<dyn Pattern>) -> TransformedPattern {
        TransformedPattern {
            delegate,
            transform: Matrix::identity(4),
        }
    }

    pub fn with_transform(self, transform: Matrix) -> PatternType {
        TransformedPattern::new(self.delegate, transform)
    }
}

impl Pattern for TransformedPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals_pattern(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn pattern_at(&self, point: Point) -> Color {
        let pattern_space_point = &self.transform.inverse() * point;
        self.delegate.pattern_at(pattern_space_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::vector::Vector;
    use crate::tracing::material::Material;
    use crate::tracing::patterns::pattern::{Pattern, TransformedPattern};
    use crate::tracing::patterns::stripe_pattern::StripePattern;
    use crate::tracing::point_light::PointLight;
    use crate::tracing::shapes::shape::Shape;
    use crate::tracing::shapes::sphere::Sphere;
    use crate::tracing::test_helpers::TestPattern;
    use std::any::Any;

    // high level tests copied from original StripedPattern

    #[test]
    fn stripes_with_object_transform() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        let object = Shape::sphere()
            .with_transform(transformations::scaling(2, 2, 2))
            .with_material(Material::default().with_pattern(pattern).with_ambient(1.0));

        let color = object.lighting(
            &PointLight::default(),
            Point::at(1.5, 0.0, 0.0),
            Vector::new(10, -10, 10),
            Vector::new(0, 0, 0),
            true,
        );
        assert_eq!(Color::WHITE, color)
    }

    #[test]
    fn stripes_with_pattern_transform() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK)
            .with_transform(transformations::scaling(2, 2, 2));

        let object = Shape::sphere()
            .with_material(Material::default().with_pattern(pattern).with_ambient(1.0));

        let color = object.lighting(
            &PointLight::default(),
            Point::at(1.5, 0.0, 0.0),
            Vector::new(10, -10, 10),
            Vector::new(0, 0, 0),
            true,
        );
        assert_eq!(Color::WHITE, color)
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transform() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK)
            .with_transform(transformations::translation(0.5, 0.0, 0.0));

        let object = Shape::sphere()
            .with_transform(transformations::scaling(2, 2, 2))
            .with_material(Material::default().with_pattern(pattern).with_ambient(1.0));

        let color = object.lighting(
            &PointLight::default(),
            Point::at(1.5, 0.0, 0.0),
            Vector::new(10, -10, 10),
            Vector::new(0, 0, 0),
            true,
        );
        assert_eq!(Color::WHITE, color)
    }

    // New tests over generic Pattern

    #[test]
    fn pattern_with_transform() {
        let pattern = TestPattern {}.with_transform(transformations::scaling(2, 2, 2));
        let color = pattern.pattern_at(Point::at(2, 3, 4));
        assert_eq!(Color::new(1.0, 1.5, 2.0), color);
    }
}
