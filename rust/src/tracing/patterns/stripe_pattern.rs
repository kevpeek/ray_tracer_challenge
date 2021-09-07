use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::tracing::patterns::pattern::{Pattern, PatternType, TransformedPattern};
use std::any::Any;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
}

impl StripePattern {
    pub fn new(color_a: Color, color_b: Color) -> PatternType {
        TransformedPattern::using_identity(Box::new(StripePattern { color_a, color_b }))
    }
}

impl Pattern for StripePattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals_pattern(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn pattern_at(&self, point: Point) -> Color {
        match point.x % 2.0 {
            x if x >= 0.0 && x < 1.0 => self.color_a,
            x if x >= 1.0 => self.color_b,
            x if x < 0.0 && x >= -1.0 => self.color_b,
            _ => self.color_a,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::vector::Vector;
    use crate::tracing::material::Material;
    use crate::tracing::patterns::pattern::Pattern;
    use crate::tracing::patterns::stripe_pattern::StripePattern;
    use crate::tracing::point_light::PointLight;
    use crate::tracing::shapes::sphere::Sphere;

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        let point = Point::at(0, 0, 0);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
        let point = Point::at(0, 1, 0);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
        let point = Point::at(0, 2, 0);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        let point = Point::at(0, 0, 0);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
        let point = Point::at(0, 0, 1);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
        let point = Point::at(0, 0, 2);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        let point = Point::at(0, 0, 0);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
        let point = Point::at(0.9, 0.0, 0.0);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
        let point = Point::at(1, 0, 0);
        assert_eq!(Color::BLACK, pattern.pattern_at(point));

        let point = Point::at(-0.1, 0.0, 0.0);
        assert_eq!(Color::BLACK, pattern.pattern_at(point));
        let point = Point::at(-1, 0, 0);
        assert_eq!(Color::BLACK, pattern.pattern_at(point));
        let point = Point::at(-1.1, 0.0, 0.0);
        assert_eq!(Color::WHITE, pattern.pattern_at(point));
    }
}
