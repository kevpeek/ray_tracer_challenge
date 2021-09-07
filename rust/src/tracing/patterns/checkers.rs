use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::tracing::patterns::pattern::{Pattern, PatternType, TransformedPattern};
use std::any::Any;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Checkers {
    color_one: Color,
    color_two: Color,
}

impl Checkers {
    pub fn new(color_one: Color, color_two: Color) -> PatternType {
        TransformedPattern::using_identity(Box::new(Checkers {
            color_one,
            color_two,
        }))
    }
}

impl Pattern for Checkers {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals_pattern(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn pattern_at(&self, point: Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            self.color_one
        } else {
            self.color_two
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::tracing::patterns::checkers::Checkers;
    use crate::tracing::patterns::pattern::Pattern;

    #[test]
    fn checkers_repeat_in_x() {
        let pattern = Checkers::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0, 0, 0)));
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0.99, 0.0, 0.0)));
        assert_eq!(Color::BLACK, pattern.pattern_at(Point::at(1.01, 0.0, 0.0)));
    }

    #[test]
    fn checkers_repeat_in_y() {
        let pattern = Checkers::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0.0, 0.0, 0.0)));
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0.0, 0.99, 0.0)));
        assert_eq!(Color::BLACK, pattern.pattern_at(Point::at(0.0, 1.01, 0.0)));
    }

    #[test]
    fn checkers_repeat_in_z() {
        let pattern = Checkers::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0.0, 0.0, 0.0)));
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0.0, 0.0, 0.99)));
        assert_eq!(Color::BLACK, pattern.pattern_at(Point::at(0.0, 0.0, 1.01)));
    }
}
