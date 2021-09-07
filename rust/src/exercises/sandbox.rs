use crate::display::color::Color;
use crate::geometry::transformations;
use crate::tracing::material::Material;
use crate::tracing::patterns::checkers::Checkers;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
use crate::tracing::shapes::sphere::Sphere;
use crate::tracing::world::World;
use crate::exercises::material_helpers;

pub fn make_world() -> World {
    let light_source = PointLight::default();

    // ===== Floor =====

    let board_material = Material::default()
        .with_pattern(Checkers::new(Color::WHITE, Color::BLACK))
        .with_specular(0.0);

    let board = Shape::plane().with_material(board_material.clone());

    // ===== Spheres =====

    let pawn_scaling = transformations::scaling(0.25, 0.5, 0.25);
    let back_row_scaling = transformations::scaling(0.25, 0.75, 0.25);

    let mut objects: Vec<Shape> = vec![board];

    for z in -1..7 {
        let z = z as f64;
        let front_row_translation = transformations::translation(-2.5, 0.5, z + 0.5, );
        let back_row_translation = transformations::translation(-3.5, 0.5, z + 0.5, );
        objects.push(
            Shape::sphere()
                .with_transform(pawn_scaling.then(&front_row_translation))
                .with_material(material_helpers::glass()),
        );
        objects.push(
            Shape::sphere()
                .with_transform(back_row_scaling.then(&back_row_translation))
                .with_material(material_helpers::glass()),
        );
    }

    for z in -1..7 {
        let z = z as f64;
        let front_row_translation = transformations::translation(2.5, 0.5, z + 0.5, );
        let back_row_translation = transformations::translation(3.5, 0.5, z + 0.5, );
        objects.push(
            Shape::sphere()
                .with_transform(pawn_scaling.then(&front_row_translation))
                .with_material(material_helpers::colored_glass(Color::WHITE)),
        );
        objects.push(
            Shape::sphere()
                .with_transform(back_row_scaling.then(&back_row_translation))
                .with_material(material_helpers::colored_glass(Color::WHITE)),
        );
    }

    World::new(objects, light_source)
}
