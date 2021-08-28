use crate::tracing::world::World;
use crate::tracing::shapes::shape::Shape;
use crate::geometry::transformations::{scaling, translation};
use crate::tracing::shapes::sphere::Sphere;
use crate::tracing::point_light::PointLight;
use crate::tracing::material::Material;
use crate::tracing::patterns::stripe_pattern::StripePattern;
use crate::display::color::Color;
use crate::geometry::transformations;
use std::f64::consts::PI;
use crate::tracing::shapes::plane::Plane;
use rand::prelude::*;
use crate::geometry::point::Point;
use std::ops::{Sub, Not};

pub fn make_world() -> World {
    let light_source = PointLight::default();

    // ===== Wall =====

    let backdrop_material = Material::default()
        .with_pattern(StripePattern::solid_pattern(Color::LIGHT_BLUE))
        .with_ambient(0.75);

    let backdrop_transform = transformations::rotation_x(PI / 2.0)
        .then(&translation(0, 0, 10));

    let backdrop = Plane::new()
        .with_material(backdrop_material)
        .with_transform(backdrop_transform);

    // ===== Spheres =====

    // let seed = thread_rng().gen_range(0..99999);
    let seed = 29202;
    println!("The seed for this render was {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    let mut objects: Vec<Box<dyn Shape>> = vec![
        Box::new(backdrop),
    ];

    let sphere_details = make_sphere_origins(&mut rng);
    for (origin, radius) in sphere_details {
        let sphere_material = random_material(&mut rng);

        let sphere = Sphere::default()
            .with_material(sphere_material)
            .with_transform(
                transformations::scaling(radius, radius, radius)
                    .then(&transformations::translation(origin.x, origin.y, origin.z))
            );

        objects.push(Box::new(sphere));
    }

    World::new(objects, light_source)
}

fn make_sphere_origins(rng: &mut StdRng) -> Vec<(Point, f64)> {
    let target = 10;
    let max = 250;

    let mut sphere_details = Vec::new();
    for _ in 0..max {
        let new_point = Point::at(
            rng.gen_range(-7.25..7.25),
            rng.gen_range(-3.25..3.75),
            rng.gen_range(7.0..10.0)
        );
        let new_radius = rng.gen_range(0.2..0.75);

        if !sphere_details.iter()
            .any(
                |(existing, radius): &(Point, f64)| new_point.sub(*existing).magnitude() < (new_radius + radius)
            ) {
                sphere_details.push((new_point, new_radius));
                if sphere_details.len() >= target {
                    break;
                }
        }
    }
    println!("{} spheres created", sphere_details.len());
    sphere_details
}

fn random_material(rng: &mut StdRng) -> Material {
    let colors = vec![Color::MUSTARD_YELLOW, Color::LIGHT_BLUE, Color::LIGHT_GREEN, Color::RED];
    let color_a = *colors.choose(rng).unwrap();
    let color_b = *colors.choose(rng).unwrap();

    let pattern = StripePattern::new(color_a, color_b)
        .with_transform(
            transformations::scaling(0.2, 0.2, 0.2)
                .then(&transformations::rotation_y(rng.gen_range(0.0..PI/2.0)))
                .then(&transformations::rotation_z(rng.gen_range(0.0..PI/2.0)))
        );

    Material::default()
        .with_pattern(pattern)
        .with_diffuse(0.7)
        .with_specular(0.3)
}
