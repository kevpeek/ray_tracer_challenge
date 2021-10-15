use crate::display::color::Color;
use crate::exercises::material_helpers;
use crate::geometry::point::Point;
use crate::geometry::transformations;
use crate::geometry::vector::Vector;
use crate::tracing::camera::{Camera, CameraMaker};
use crate::tracing::material::Material;
use crate::tracing::patterns::checkers::Checkers;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::cube::Cube;
use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
use crate::tracing::world::World;
use std::f64::consts::PI;

pub fn make_world() -> (World, CameraMaker) {
    let light_source = PointLight::default();

    // ===== Base =====

    let floor_material = Material::default().with_color(Color::LIGHT_BLUE);
    let floor = Shape::plane().with_material(floor_material);

    let board_material = Material::default()
        .with_pattern(
            Checkers::new(Color::WHITE, Color::BLACK)
                .with_transform(transformations::scaling(0.25, 1.0, 0.25)),
        )
        .with_specular(0.0)
        .with_reflective(0.5);

    let board = Cube::new()
        .into_shape()
        .with_material(board_material)
        .with_transform(transformations::scaling(4.0, 0.25, 4.0));

    let mut objects: Vec<Shape> = vec![floor, board];

    // ===== Pieces =====

    let pawn_scaling = transformations::scaling(0.25, 0.5, 0.25);
    let back_row_scaling = transformations::scaling(0.25, 0.75, 0.25);

    let pawn_elevation = 0.75;
    let back_row_elevation = 1.0;

    let rotation = transformations::rotation_y(PI / 2.0);

    objects.push(
        Shape::sphere()
            .with_transform(
                pawn_scaling
                    .then(&transformations::translation(1.5, pawn_elevation, -2.5))
                    .then(&rotation),
            )
            .with_material(material_helpers::colored_glass(Color::WHITE)),
    );

    objects.push(
        Shape::sphere()
            .with_transform(
                pawn_scaling
                    .then(&transformations::translation(2.5, pawn_elevation, -0.5))
                    .then(&rotation),
            )
            .with_material(material_helpers::colored_glass(Color::RED)),
    );

    objects.push(
        Shape::sphere()
            .with_transform(
                pawn_scaling
                    .then(&transformations::translation(1.5, pawn_elevation, 1.5))
                    .then(&rotation),
            )
            .with_material(material_helpers::colored_glass(Color::RED)),
    );

    objects.push(
        Shape::sphere()
            .with_transform(
                back_row_scaling
                    .then(&transformations::translation(0.5, back_row_elevation, 0.5))
                    .then(&rotation),
            )
            .with_material(material_helpers::colored_glass(Color::RED)),
    );

    objects.push(
        Shape::sphere()
            .with_transform(
                back_row_scaling
                    .then(&transformations::translation(2.5, back_row_elevation, 1.5))
                    .then(&rotation),
            )
            .with_material(material_helpers::colored_glass(Color::WHITE)),
    );

    (World::new(objects, light_source), camera_maker())
}

fn camera_maker() -> CameraMaker {
    let camera_transform = transformations::view_transform(
        Point::at(-2.0, 3.5, -10.0),
        Point::at(0, 1, 0),
        Vector::new(0, 1, 0),
    );
    Box::new(|resolution| Camera::new(resolution, PI / 3.0, camera_transform))
}
