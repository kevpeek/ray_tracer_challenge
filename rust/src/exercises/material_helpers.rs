use crate::tracing::material::Material;
use crate::display::color::Color;

pub fn glass() -> Material {
    colored_glass(Color::BLACK)
}

pub fn colored_glass(color: Color) -> Material {
    Material::default()
        .with_color(color)
        .with_transparency(1.0)
        .with_refractive_index(1.5)
        .with_reflective(0.9)
        .with_ambient(0.1)
        .with_diffuse(0.1)
        .with_specular(300.0)
}
