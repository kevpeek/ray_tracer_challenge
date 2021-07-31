use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::almost;
use crate::tracing::point_light::PointLight;

pub fn lighting(
    material: &Material,
    light: &PointLight,
    position: Point,
    eye_vector: Vector,
    normal: Vector,
    in_shadow: bool,
) -> Color {
    let ambient = ambient_contribution(material, light);
    let diffuse = match in_shadow {
        true => Color::BLACK,
        false => diffuse_contribution(material, light, position, normal)
    };
    let specular = match in_shadow {
        true => Color::BLACK,
        false => specular_contribution(material, light, position, eye_vector, normal)
    };
    ambient + diffuse + specular
}

fn ambient_contribution(material: &Material, light: &PointLight) -> Color {
    let effective_color = effective_color(material, light);
    effective_color * material.ambient
}

fn diffuse_contribution(
    material: &Material,
    light: &PointLight,
    position: Point,
    normal: Vector,
) -> Color {
    let light_direction = (light.position() - position).normalize();
    let light_dot_normal = light_direction.dot(normal);
    if light_dot_normal < 0.0 {
        Color::BLACK
    } else {
        effective_color(material, light) * material.diffuse * light_dot_normal
    }
}

fn effective_color(material: &Material, light: &PointLight) -> Color {
    material.color * light.intensity()
}

fn specular_contribution(
    material: &Material,
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
            let factor = reflect_dot_eye.powf(material.shininess);
            light.intensity() * material.specular * factor
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn with_color(&self, color: Color) -> Material {
        Material::new(
            color,
            self.ambient,
            self.diffuse,
            self.specular,
            self.shininess,
        )
    }

    pub fn with_ambient(&self, ambient: f64) -> Material {
        Material::new(
            self.color,
            ambient,
            self.diffuse,
            self.specular,
            self.shininess,
        )
    }

    pub fn with_diffuse(&self, diffuse: f64) -> Material {
        Material::new(
            self.color,
            self.ambient,
            diffuse,
            self.specular,
            self.shininess,
        )
    }

    pub fn with_specular(&self, specular: f64) -> Material {
        Material::new(
            self.color,
            self.ambient,
            self.diffuse,
            specular,
            self.shininess,
        )
    }

    pub fn with_shininess(&self, shininess: f64) -> Material {
        Material::new(
            self.color,
            self.ambient,
            self.diffuse,
            self.specular,
            shininess,
        )
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
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
    use crate::tracing::material::{lighting, Material};
    use crate::tracing::point_light::PointLight;

    #[test]
    fn default_material() {
        assert_eq!(Color::new(1, 1, 1), Material::default().color);
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

        let result = lighting(
            &Material::default(),
            &light,
            Point::origin(),
            eye_vector,
            normal,
            false,
        );
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let eye_vector = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, -10), Color::new(1, 1, 1));

        let result = lighting(
            &Material::default(),
            &light,
            Point::origin(),
            eye_vector,
            normal,
            false,
        );
        assert_eq!(Color::new(1, 1, 1), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 10, -10), Color::new(1, 1, 1));

        let result = lighting(
            &Material::default(),
            &light,
            Point::origin(),
            eye_vector,
            normal,
            false,
        );
        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let eye_vector = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 10, -10), Color::new(1, 1, 1));

        let result = lighting(
            &Material::default(),
            &light,
            Point::origin(),
            eye_vector,
            normal,
            false,
        );
        assert_eq!(Color::new(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, 10), Color::new(1, 1, 1));

        let result = lighting(
            &Material::default(),
            &light,
            Point::origin(),
            eye_vector,
            normal,
            false,
        );
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let eye_vector = Vector::new(0, 0, -1);
        let normal = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::at(0, 0, -10), Color::new(1, 1, 1));
        let in_shadow = true;

        let result = lighting(
            &Material::default(),
            &light,
            Point::origin(),
            eye_vector,
            normal,
            in_shadow
        );
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
}
