use crate::display::color::Color;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::tracing::patterns::pattern::{Pattern, TransformedPattern};
use std::any::Any;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub(crate) fn with_transform(self, transform: Matrix) -> TransformedPattern {
        TransformedPattern::new(Box::new(self), transform)
    }

    pub fn without_transform(self) -> TransformedPattern {
        TransformedPattern::new(Box::new(self), Matrix::identity(4))
    }
}

impl Pattern for TestPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals_pattern(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn pattern_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}
