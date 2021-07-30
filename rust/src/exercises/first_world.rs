use crate::display::color::Color;
use crate::tracing::material::Material;
use crate::tracing::sphere::Sphere;
use crate::geometry::point::Point;
use crate::geometry::transformations::{scaling, rotation_x, rotation_y, translation, view_transform};
use std::f64::consts::PI;
use crate::tracing::point_light::PointLight;
use crate::tracing::world::World;
use crate::geometry::vector::Vector;
use crate::tracing::camera::Camera;
use crate::display::writer::write_canvas;

pub fn run() {
    let wall_material = Material::default()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Sphere::new(
        Point::origin(),
        wall_material.clone(),
        scaling(10.0, 0.01, 10.0)
    );

    let transform = scaling(10.0, 0.01, 10.0)
        .then(&rotation_x(PI / 2.0))
        .then(&rotation_y(-PI / 4.0))
        .then(&translation(0, 0, 5));

    let leftWall = Sphere::new(Point::origin(), wall_material.clone(), transform.clone());

    let rightWall = Sphere::new(Point::origin(), wall_material.clone(), transform.clone());

    let middleTransform = translation(-0.5, 1.0, 0.5);
    let middleMaterial = Material::default()
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3);

    let middle = Sphere::new(Point::origin(), middleMaterial.clone(), middleTransform);

    let rightTransform = scaling(0.5, 0.5, 0.5)
        .then(&translation(1.5, 0.5, -0.5));

    let right = Sphere::new(Point::origin(), middleMaterial, rightTransform);

    let leftTransform = scaling(0.33, 0.33, 0.33)
        .then(&translation(-1.5, 0.33, -0.75));

    let leftMaterial = Material::default()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);

    let left = Sphere::new(Point::origin(), leftMaterial, leftTransform);

    let lightSource = PointLight::new(Point::at(-10, 10, -10), Color::new(1, 1, 1));
    let world = World::new(vec![floor, leftWall, rightWall, middle, right, left], lightSource);

    let cameraTransform = view_transform(Point::at(0.0, 1.5, -5.0), Point::at(0, 1, 0), Vector::new(0, 1, 0));
    let camera = Camera::new(400, 200, PI / 3.0, cameraTransform);

    let canvas = camera.render(world);
    write_canvas(&canvas);
}
