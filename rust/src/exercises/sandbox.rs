use crate::display::color::Color;
use crate::tracing::material::Material;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
use crate::tracing::world::World;
use crate::geometry::transformations;
use crate::tracing::patterns::checkers::Checkers;
use crate::tracing::shapes::sphere::Sphere;


pub fn make_world() -> World {
    let light_source = PointLight::default();

    // ===== Floor =====

    let wall_material = Material::default()
        .with_pattern(Checkers::new(Color::WHITE, Color::BLACK))
        .with_specular(0.0);

    let floor = Shape::plane()
        .with_material(wall_material.clone());

    // ===== Spheres =====

    let glass = Material::default()
        .with_color(Color::BLACK)
        .with_transparency(1.0)
        .with_refractive_index(1.5)
        .with_ambient(0.0)
        .with_diffuse(0.0)
        .with_specular(0.0);
    let middle = Shape::sphere()
        .with_transform(transformations::translation(-0.5, 1.0, 0.5))
        .with_material(glass.clone());

    let middle_material = Material::default()
        .with_color(Color::LIGHT_GREEN)
        .with_diffuse(0.7)
        .with_specular(0.3);

    let back = Sphere::new().into_shape()
        .with_material(middle_material)
        .with_transform(transformations::translation(0, 1, 10));

    let objects: Vec<Shape> = vec![
        floor,
        middle,
        back,
    ];

    World::new(objects, light_source)
}
