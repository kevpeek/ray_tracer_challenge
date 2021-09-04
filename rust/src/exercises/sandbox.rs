use std::f64::consts::PI;

use crate::display::color::Color;
use crate::geometry::transformations::{
    rotation_x, rotation_y, scaling, translation,
};
use crate::tracing::material::Material;
use crate::tracing::shapes::plane::Plane;
use crate::tracing::point_light::PointLight;
use crate::tracing::shapes::shape::Shape;
use crate::tracing::shapes::sphere::Sphere;
use crate::tracing::world::World;
use crate::tracing::patterns::stripe_pattern::StripePattern;
use crate::geometry::transformations;
use crate::tracing::patterns::gradient::Gradient;
use crate::tracing::patterns::checkers::Checkers;
use crate::geometry::matrix::Matrix;


pub fn make_world() -> World {
    let light_source = PointLight::default();

    // ===== Floor =====

    let wall_material = Material::default()
        .with_pattern(Checkers::new(Color::WHITE, Color::BLACK))
        .with_specular(0.0);

    let floor = Shape::plane()
        .with_material(wall_material.clone());

    // ===== Spheres =====

    let middle_material = Material::default()
        .with_color(Color::LIGHT_GREEN)
        .with_diffuse(0.7)
        .with_specular(0.3);
    let middle = Shape::sphere()
        .with_transform(translation(-0.5, 1.0, 0.5))
        .with_material(middle_material.clone());

    let objects: Vec<Shape> = vec![
        floor,
        middle,
    ];

    World::new(objects, light_source)
}
