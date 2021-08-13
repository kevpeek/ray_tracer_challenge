use crate::display::color::Color;
use crate::display::resolution::Resolution;
use crate::display::writer::write_canvas;
use crate::geometry::point::Point;
use crate::geometry::transformations::{
    rotation_x, rotation_y, scaling, translation, view_transform,
};
use crate::geometry::vector::Vector;
use crate::tracing::camera::Camera;
use crate::tracing::material::Material;
use crate::tracing::point_light::PointLight;
use crate::tracing::sphere::Sphere;
use crate::tracing::world::World;
use crate::tracing::shape::{Shape, WorldShape};
use std::f64::consts::PI;

pub fn run_world() {
    let world = world_one();
    let camera = make_camera();

    let canvas = camera.render(world);
    write_canvas(&canvas).unwrap();
}

fn make_camera() -> Camera {
    let camera_transform = view_transform(
        Point::at(0.0, 1.5, -5.0),
        Point::at(0, 1, 0),
        Vector::new(0, 1, 0),
    );
    let camera = Camera::new(Resolution::LOW, PI / 3.0, camera_transform);
    camera
}

fn world_one() -> World {
    let light_source = PointLight::default();

    // ===== Walls =====

    let wall_material = Material::default()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Sphere::default()
        .with_material(wall_material.clone())
        .with_transform(scaling(10.0, 0.01, 10.0));

    let wall_transform = scaling(10.0, 0.01, 10.0)
        .then(&rotation_x(PI / 2.0))
        .then(&translation(0, 0, 5));

    let left_wall = Sphere::default()
        .with_material(wall_material.clone())
        .with_transform(wall_transform.then(&rotation_y(-PI / 4.0)));

    let right_wall = Sphere::default()
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
        .with_color(Color::MUSTARD_YELLOW)
        .with_diffuse(0.7)
        .with_specular(0.3);

    let left = Sphere::default()
        .with_material(left_material)
        .with_transform(scaling(0.33, 0.33, 0.33).then(&translation(-1.5, 0.33, -0.75)));

    let mut objects: Vec<WorldShape> = Vec::new();
    objects.push(Box::new(floor));
    objects.push(Box::new(left_wall));
    objects.push(Box::new(right_wall));
    objects.push(Box::new(middle));
    objects.push(Box::new(right));
    objects.push(Box::new(left));

    World::new(
        objects,
        light_source,
    )
}
