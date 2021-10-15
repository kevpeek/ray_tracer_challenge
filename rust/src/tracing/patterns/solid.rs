use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::tracing::patterns::pattern::{Pattern, PatternType, TransformedPattern};
use std::any::Any;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Color) -> PatternType {
        TransformedPattern::using_identity(Box::new(Solid { color }))
    }
}

impl Pattern for Solid {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals_pattern(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn pattern_at(&self, _: Point) -> Color {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::tracing::patterns::pattern::Pattern;
    
    use crate::tracing::patterns::solid::Solid;

    #[test]
    fn rings_extend_in_both_x_and_z() {
        let pattern = Solid::new(Color::WHITE);
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0, 0, 0)));
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(1, 0, 0)));
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0, 0, 1)));
    }
}
