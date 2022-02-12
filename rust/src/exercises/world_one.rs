use std::f64::consts::PI;

use crate::display::color::Color;
use crate::exercises::snapshot;
use crate::geometry::transformations;
use crate::geometry::transformations::{scaling, translation};
use crate::tracing::camera::CameraMaker;
use crate::tracing::material::Material;
use crate::tracing::patterns::checkers::Checkers;
use crate::tracing::patterns::gradient::Gradient;
use crate::tracing::patterns::stripe_pattern::StripePattern;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::shape::Shape;
use crate::tracing::world::World;

pub fn make_world() -> (World, CameraMaker) {
    let light_source = PointLight::default();

    // ===== Walls =====

    let wall_material = Material::default()
        .with_pattern(Checkers::new(Color::WHITE, Color::BLACK))
        .with_specular(0.0)
        .with_reflective(1.0);

    let floor = Shape::plane().with_material(wall_material.clone());

    // ===== Spheres =====

    let middle_material = Material::default()
        .with_color(Color::PURPLE)
        .with_diffuse(0.7)
        .with_specular(0.3);
    let middle = Shape::sphere()
        .with_transform(translation(-0.5, 1.0, 0.5))
        .with_material(middle_material.clone());

    let right = Shape::sphere()
        .with_transform(scaling(0.5, 0.5, 0.5).then(&translation(1.5, 0.5, -0.5)))
        .with_material(middle_material.with_pattern(Gradient::new(Color::LIGHT_BLUE, Color::RED)));

    let left_material = Material::default()
        .with_pattern(
            StripePattern::new(Color::MUSTARD_YELLOW, Color::BLACK).with_transform(
                transformations::scaling(0.2, 0.2, 0.2)
                    .then(&transformations::rotation_z(PI / 2.0)),
            ),
        )
        .with_diffuse(0.7)
        .with_specular(0.3);

    let left = Shape::sphere()
        .with_transform(scaling(0.33, 0.33, 0.33).then(&translation(-1.5, 0.33, -0.75)))
        .with_material(left_material);

    let objects: Vec<Shape> = vec![
        floor, // left_wall,
        // right_wall,
        middle, right, left,
    ];

    (
        World::new(objects, light_source),
        snapshot::camera_one_maker(),
    )
}
