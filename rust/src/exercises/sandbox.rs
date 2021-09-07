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
        .with_color(Color::RED)
        .with_transparency(1.0)
        .with_refractive_index(1.5)
        .with_reflective(0.9)
        .with_ambient(0.1)
        .with_diffuse(0.1)
        .with_specular(300.0);

    let scaling = transformations::scaling(0.25, 0.5, 0.25);

    let mut objects: Vec<Shape> = vec![floor];

    for x in -3..3 {
        objects.push(Shape::sphere()
            .with_transform(scaling.then(&transformations::translation(x as f64 + 0.5, 0.5, 0.5)))
            .with_material(glass.clone()));
    }


    World::new(objects, light_source)
}
