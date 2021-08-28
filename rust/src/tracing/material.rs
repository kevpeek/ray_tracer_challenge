use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::almost;
use crate::tracing::point_light::PointLight;
use crate::tracing::patterns::stripe_pattern::StripePattern;
use crate::tracing::patterns::pattern::{Pattern, PatternType};

#[derive(Debug, Clone)]
pub struct Material {
    pattern: PatternType,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn default() -> Material {
        Material {
            pattern: StripePattern::solid_pattern(Color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn new(
        pattern: PatternType,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Material {
        Material {
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn solid_colored(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Material {
        let pattern= StripePattern::solid_pattern(color);
        Material {
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn with_color(self, color: Color) -> Material {
        Material::solid_colored(
            color,
            self.ambient,
            self.diffuse,
            self.specular,
            self.shininess,
        )
    }

    pub fn with_pattern(self, pattern: PatternType) -> Material {
        Material::new(
            pattern,
            self.ambient,
            self.diffuse,
            self.specular,
            self.shininess,
        )
    }

    pub fn with_ambient(self, ambient: f64) -> Material {
        Material::new(
            self.pattern,
            ambient,
            self.diffuse,
            self.specular,
            self.shininess,
        )
    }

    pub fn with_diffuse(self, diffuse: f64) -> Material {
        Material::new(
            self.pattern,
            self.ambient,
            diffuse,
            self.specular,
            self.shininess,
        )
    }

    pub fn with_specular(self, specular: f64) -> Material {
        Material::new(
            self.pattern,
            self.ambient,
            self.diffuse,
            specular,
            self.shininess,
        )
    }

    pub fn with_shininess(self, shininess: f64) -> Material {
        Material::new(
            self.pattern,
            self.ambient,
            self.diffuse,
            self.specular,
            shininess,
        )
    }

    pub fn color(&self) -> Color {
        let point = Point::origin();
        self.pattern.pattern_at(point)
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: Point,
        eye_vector: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        let point = position;
        let effective_color = self.pattern.pattern_at(point) * light.intensity();

        let ambient = self.ambient_contribution(effective_color);
        let diffuse = match in_shadow {
            true => Color::BLACK,
            false => self.diffuse_contribution(light, position, normal, effective_color),
        };
        let specular = match in_shadow {
            true => Color::BLACK,
            false => self.specular_contribution(light, position, eye_vector, normal),
        };
        ambient + diffuse + specular
    }

    fn ambient_contribution(&self, effective_color: Color) -> Color {
        effective_color * self.ambient
    }

    fn diffuse_contribution(
        &self,
        light: &PointLight,
        position: Point,
        normal: Vector,
        effective_color: Color,
    ) -> Color {
        let light_direction = (light.position() - position).normalize();
        let light_dot_normal = light_direction.dot(normal);
        if light_dot_normal < 0.0 {
            Color::BLACK
        } else {
            effective_color * self.diffuse * light_dot_normal
        }
    }

    fn specular_contribution(
        &self,
        light: &PointLight,
        position: Point,
        eye_vector: Vector,
        normal: Vector,
    ) -> Color {
        let light_direction = (light.position() - position).normalize();
        let light_dot_normal = light_direction.dot(normal);
        if light_dot_normal < 0.0 {
            Color::BLACK
        } else {
            let reflect_vector = -light_direction.reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);

            if reflect_dot_eye < 0.0 {
                Color::BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity() * self.specular * factor
            }
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
            && almost(self.ambient, other.ambient)
            && almost(self.diffuse, other.diffuse)
            && almost(self.specular, other.specular)
            && almost(self.shininess, other.shininess)
    }
}
impl Eq for Material {}

#[cfg(test)]
mod test {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::geometry::vector::Vector;
    use crate::tracing::material::{Material};
    use crate::tracing::point_light::PointLight;
    use crate::tracing::patterns::stripe_pattern::StripePattern;

    #[test]
    fn default_material() {
        assert_eq!(Color::new(1, 1, 1), Material::default().color());
        assert_eq!(0.1, Material::default().ambient);
        assert_eq!(0.9, Material::default().diffuse);
        assert_eq!(0.9, Material::default().specular);
        assert_eq!(200.0, Material::default().shininess);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, -10), Color::new(1, 1, 1));

        let material = &Material::default();
        let light_argument = &light;
        let position = Point::origin();
        let in_shadow = false;
        let result = material.lighting(light_argument, position, eye_vector, normal, in_shadow);
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let eye_vector = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, -10), Color::new(1, 1, 1));

        let material = &Material::default();
        let light_argument = &light;
        let position = Point::origin();
        let in_shadow = false;
        let result = material.lighting(light_argument, position, eye_vector, normal, in_shadow);
        assert_eq!(Color::new(1, 1, 1), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 10, -10), Color::new(1, 1, 1));

        let material = &Material::default();
        let light_argument = &light;
        let position = Point::origin();
        let in_shadow = false;
        let result = material.lighting(light_argument, position, eye_vector, normal, in_shadow);
        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let eye_vector = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 10, -10), Color::new(1, 1, 1));

        let material = &Material::default();
        let light_argument = &light;
        let position = Point::origin();
        let in_shadow = false;
        let result = material.lighting(light_argument, position, eye_vector, normal, in_shadow);
        assert_eq!(Color::new(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, 10), Color::new(1, 1, 1));

        let material = &Material::default();
        let light_argument = &light;
        let position = Point::origin();
        let in_shadow = false;
        let result = material.lighting(light_argument, position, eye_vector, normal, in_shadow);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, -10), Color::new(1, 1, 1));
        let in_shadow = true;

        let material = &Material::default();
        let light_argument = &light;
        let position = Point::origin();
        let result = material.lighting(light_argument, position, eye_vector, normal, in_shadow);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_pattern() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        let material = Material::new(pattern, 1.0, 0.0, 0.0, 200.0);
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, -10), Color::WHITE);
        let material_argument = &material;
        let light_argument = &light;
        let position = Point::at(0.9, 0.0, 0.0);
        let in_shadow = false;
        let color_one = material_argument.lighting(light_argument, position, eye_vector, normal, in_shadow);
        let material_argument = &material;
        let light_argument = &light;
        let position = Point::at(1.1, 0.0, 0.0);
        let in_shadow = false;
        let color_two = material_argument.lighting(light_argument, position, eye_vector, normal, in_shadow);
        assert_eq!(Color::WHITE, color_one);
        assert_eq!(Color::BLACK, color_two);
    }
}
