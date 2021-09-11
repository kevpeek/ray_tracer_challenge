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
use crate::tracing::shapes::cylinder::Cylinder;

pub fn make_world() -> World {
    let light_source = PointLight::default();

    // ===== Base =====

    let floor_material = Material::default()
        .with_pattern(Checkers::new(Color::WHITE, Color::BLACK));
    let floor = Shape::plane().with_material(floor_material);


    // ===== Spheres =====


    let object_material = Material::default()
        .with_color(Color::LIGHT_GREEN)
        .with_diffuse(0.7)
        .with_specular(100.00)
        .with_shininess(300.0)
        .with_reflective(1.0);
    let object = Cylinder::new(0.0, 1.0).into_shape()
        // .with_transform(transformations::translation(-0.5, 0.0, 0.5))
        .with_material(object_material);

    let mut objects: Vec<Shape> = vec![floor, object];

    World::new(objects, light_source)
}
