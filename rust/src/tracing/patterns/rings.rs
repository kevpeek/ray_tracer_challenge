use crate::display::color::Color;
use crate::tracing::patterns::pattern::{PatternType, Pattern, TransformedPattern};
use std::any::Any;
use crate::geometry::point::Point;
use crate::geometry::matrix::Matrix;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rings {
    color_one: Color,
    color_two: Color
}

impl Rings {
    pub fn new(color_one: Color, color_two: Color) -> PatternType {
        TransformedPattern::using_identity(Box::new(Rings { color_one, color_two }))
    }
}

impl Pattern for Rings {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals_pattern(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn pattern_at(&self, point: Point) -> Color {
        if (point.x * point.x + point.z * point.z).sqrt().floor() % 2.0 == 0.0 {
            self.color_one
        } else {
            self.color_two
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tracing::patterns::rings::Rings;
    use crate::display::color::Color;
    use crate::tracing::patterns::pattern::Pattern;
    use crate::geometry::point::Point;

    #[test]
    fn rings_extend_in_both_x_and_z() {
        let pattern = Rings::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.pattern_at(Point::at(0, 0, 0)));
        assert_eq!(Color::BLACK, pattern.pattern_at(Point::at(1, 0, 0)));
        assert_eq!(Color::BLACK, pattern.pattern_at(Point::at(0, 0, 1)));
        assert_eq!(Color::BLACK, pattern.pattern_at(Point::at(0.708, 0.0, 0.708)));

    }
}
