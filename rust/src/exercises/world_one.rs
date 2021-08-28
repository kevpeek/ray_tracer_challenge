use std::f64::consts::PI;

use crate::display::color::Color;
use crate::display::resolution::Resolution;
use crate::display::ppm_writer::write_canvas;
use crate::geometry::point::Point;
use crate::geometry::transformations::{
    rotation_x, rotation_y, scaling, translation, view_transform,
};
use crate::geometry::vector::Vector;
use crate::tracing::camera::Camera;
use crate::tracing::material::Material;
use crate::tracing::shapes::plane::Plane;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::shape::Shape;
use crate::tracing::shapes::sphere::Sphere;
use crate::tracing::world::World;
use crate::tracing::patterns::stripe_pattern::StripePattern;
use crate::geometry::transformations;
use crate::exercises::snapshot;


pub fn world_one() -> World {
    let light_source = PointLight::default();

    // ===== Walls =====

    let wall_material = Material::default()
        .with_pattern(StripePattern::new(Color::WHITE, Color::RED))
        .with_specular(0.0);

    let floor = Plane::new().with_material(wall_material.clone());

    let wall_transform = rotation_x(PI / 2.0).then(&translation(0, 0, 5));

    let left_wall = Plane::new()
        .with_material(wall_material.clone())
        .with_transform(wall_transform.then(&rotation_y(-PI / 4.0)));

    let right_wall = Plane::new()
        .with_material(wall_material)
        .with_transform(wall_transform.then(&rotation_y(PI / 4.0)));

    // ===== Spheres =====

    let middle_material = Material::default()
        .with_color(Color::LIGHT_GREEN)
        .with_diffuse(0.7)
        .with_specular(0.3);

    let middle = Sphere::default()
        .with_material(middle_material.clone())
        .with_transform(translation(-0.5, 1.0, 0.5));

    let right = Sphere::default()
        .with_material(middle_material.with_color(Color::LIGHT_BLUE))
        .with_transform(scaling(0.5, 0.5, 0.5).then(&translation(1.5, 0.5, -0.5)));

    let left_material = Material::default()
        // .with_color(Color::MUSTARD_YELLOW)
        .with_pattern(
            StripePattern::new(Color::MUSTARD_YELLOW, Color::BLACK)
                .with_transform(transformations::scaling(0.2, 0.2, 0.2)
                    .then(&transformations::rotation_z(PI / 2.0)))
        )
        .with_diffuse(0.7)
        .with_specular(0.3);

    let left = Sphere::default()
        .with_material(left_material)
        .with_transform(scaling(0.33, 0.33, 0.33).then(&translation(-1.5, 0.33, -0.75)));

    let objects: Vec<Box<dyn Shape>> = vec![
        Box::new(floor),
        Box::new(left_wall),
        Box::new(right_wall),
        Box::new(middle),
        Box::new(right),
        Box::new(left),
    ];

    World::new(objects, light_source)
}
