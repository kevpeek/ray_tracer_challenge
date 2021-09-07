use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::tracing::patterns::pattern::{Pattern, PatternType, TransformedPattern};
use std::any::Any;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Gradient {
    color_one: Color,
    color_two: Color,
}

impl Gradient {
    pub fn new(color_one: Color, color_two: Color) -> PatternType {
        TransformedPattern::using_identity(Box::new(Gradient {
            color_one,
            color_two,
        }))
    }
}

impl Pattern for Gradient {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals_pattern(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn pattern_at(&self, point: Point) -> Color {
        let distance = self.color_two - self.color_one;
        let fraction = (point.x.abs() - point.x.abs().floor()) * point.x.signum();
        self.color_one + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::tracing::patterns::gradient::Gradient;
    use crate::tracing::patterns::pattern::Pattern;

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0, 0, 0)));
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            pattern.pattern_at(Point::at(0.25, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            pattern.pattern_at(Point::at(0.5, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            pattern.pattern_at(Point::at(0.75, 0.0, 0.0))
        );
    }
}
