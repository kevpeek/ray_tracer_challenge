use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::tracing::shape::WorldShape;
use crate::geometry::matrix::Matrix;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
    transform: Matrix
}

impl StripePattern {
    pub fn new(color_a: Color, color_b: Color) -> StripePattern {
        StripePattern {color_a, color_b, transform: Matrix::identity(4)}
    }

    pub fn solid_pattern(color: Color) -> StripePattern {
        StripePattern::new(color, color)
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        let pattern_point = &self.transform.inverse() * point;
        match pattern_point.x % 2.0 {
            x if x >= 0.0 && x < 1.0 => self.color_a,
            x if x >= 1.0 => self.color_b,
            x if x < 0.0 && x >= -1.0 => self.color_b,
            _ => self.color_a
        }
    }

    pub fn with_transform(self, new_transform: Matrix) -> StripePattern {
        StripePattern {
            color_a: self.color_a,
            color_b: self.color_b,
            transform: new_transform,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::tracing::stripe_pattern::StripePattern;
    use crate::geometry::point::Point;
    use crate::tracing::sphere::Sphere;
    use crate::geometry::transformations;
    use crate::tracing::material::Material;
    use crate::tracing::shape::Shape;
    use crate::tracing::point_light::PointLight;
    use crate::geometry::vector::Vector;

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

    #[test]
    fn stripes_with_object_transform() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        let object = Sphere::default()
            .with_material(Material::default().with_pattern(pattern).with_ambient(1.0))
            .with_transform(transformations::scaling(2, 2, 2));

        let color = object.lighting(
            &PointLight::default(),
            Point::at(1.5, 0.0, 0.0),
            Vector::new(10, -10, 10),
            Vector::new(0, 0, 0),
            true
        );
        assert_eq!(Color::WHITE, color)
    }

    #[test]
    fn stripes_with_pattern_transform() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK)
            .with_transform(transformations::scaling(2, 2, 2));

        let object = Sphere::default()
            .with_material(Material::default().with_pattern(pattern).with_ambient(1.0));

        let color = object.lighting(
            &PointLight::default(),
            Point::at(1.5, 0.0, 0.0),
            Vector::new(10, -10, 10),
                Vector::new(0, 0, 0),
            true
        );
        assert_eq!(Color::WHITE, color)
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transform() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK)
            .with_transform(transformations::translation(0.5, 0.0, 0.0));

        let object = Sphere::default()
            .with_material(Material::default().with_pattern(pattern).with_ambient(1.0))
            .with_transform(transformations::scaling(2, 2, 2));

        let color = object.lighting(
            &PointLight::default(),
            Point::at(1.5, 0.0, 0.0),
            Vector::new(10, -10, 10),
            Vector::new(0, 0, 0),
            true
        );
        assert_eq!(Color::WHITE, color)
    }
}
