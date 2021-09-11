use crate::display::color::Color;
use crate::geometry::transformations;
use crate::tracing::material::Material;
use crate::tracing::patterns::checkers::Checkers;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
use crate::tracing::shapes::sphere::Sphere;
use crate::tracing::world::World;
use crate::exercises::material_helpers;
use crate::tracing::shapes::cube::Cube;
use crate::tracing::camera::Camera;

pub fn make_world() -> World {
    let light_source = PointLight::default();

    // ===== Base =====

    let floor_material = Material::default()
        .with_pattern(Checkers::new(Color::WHITE, Color::BLACK));
    let floor = Shape::plane().with_material(floor_material);


    // ===== Spheres =====


    let ball_material = Material::default()
        .with_color(Color::LIGHT_GREEN)
        .with_diffuse(0.7)
        .with_specular(10.00)
        .with_shininess(100.0)
        .with_reflective(1.0);
    let ball = Shape::sphere()
        .with_transform(transformations::translation(-0.5, 1.0, 0.5))
        .with_material(ball_material);

    let mut objects: Vec<Shape> = vec![floor, ball];

    World::new(objects, light_source)
}
