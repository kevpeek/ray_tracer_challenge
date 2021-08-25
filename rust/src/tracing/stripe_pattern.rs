use crate::display::color::Color;
use crate::geometry::point::Point;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
}

impl StripePattern {
    pub fn new(color_a: Color, color_b: Color) -> StripePattern {
        StripePattern {color_a, color_b}
    }

    pub fn solid_pattern(color: Color) -> StripePattern {
        StripePattern::new(color, color)
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        match point.x % 2.0 {
            x if x >= 0.0 && x < 1.0 => self.color_a,
            x if x >= 1.0 => self.color_b,
            x if x < 0.0 && x >= -1.0 => self.color_b,
            _ => self.color_a
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::tracing::stripe_pattern::StripePattern;
    use crate::geometry::point::Point;

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0, 0, 0)));
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0, 1, 0)));
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0, 2, 0)));
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0, 0, 0)));
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0, 0, 1)));
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0, 0, 2)));
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0, 0, 0)));
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(0.9, 0.0, 0.0)));
        assert_eq!(Color::BLACK, pattern.stripe_at(Point::at(1, 0, 0)));

        assert_eq!(Color::BLACK, pattern.stripe_at(Point::at(-0.1, 0.0, 0.0)));
        assert_eq!(Color::BLACK, pattern.stripe_at(Point::at(-1, 0, 0)));
        assert_eq!(Color::WHITE, pattern.stripe_at(Point::at(-1.1, 0.0, 0.0)));
    }
}
