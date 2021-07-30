use crate::display::color::Color;
use crate::geometry::point::Point;

#[derive(Eq, PartialEq, Debug)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }

    pub fn default() -> PointLight {
        PointLight::new(Point::at(-10, -10, -10), Color::WHITE)
    }

    pub fn black_light() -> PointLight {
        PointLight::new(Point::origin(), Color::BLACK)
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::tracing::point_light::PointLight;

    #[test]
    fn point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::at(0, 0, 0);
        let light = PointLight::new(position, intensity);

        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
