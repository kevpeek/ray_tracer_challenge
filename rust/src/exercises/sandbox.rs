use crate::display::color::Color;
use crate::exercises::snapshot;
use crate::geometry::transformations;
use crate::tracing::camera::CameraMaker;
use crate::tracing::material::Material;
use crate::tracing::patterns::checkers::Checkers;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::cylinder::Cylinder;
use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
use crate::tracing::world::World;
use std::f64::consts::PI;

pub fn make_world() -> (World, CameraMaker) {
    let light_source = PointLight::default();

    // ===== Base =====

    let floor_material =
        Material::default().with_pattern(Checkers::new(Color::WHITE, Color::BLACK));
    let floor = Shape::plane().with_material(floor_material);

    // ===== Spheres =====

    let object_material = Material::default()
        .with_color(Color::LIGHT_GREEN)
        .with_diffuse(0.7)
        .with_specular(100.00)
        .with_shininess(300.0)
        .with_reflective(1.0);
    let object = Cylinder::new(0.0, 1.0)
        .into_shape()
        .with_transform(transformations::rotation_x(-PI / 4.0))
        .with_material(object_material);

    let objects: Vec<Shape> = vec![floor, object];

    (
        World::new(objects, light_source),
        snapshot::camera_one_maker(),
    )
}
