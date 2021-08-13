use crate::display::color::Color;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::transformations::scaling;
use crate::tracing::intersection::{
    Intersection, Intersections, PreComputedIntersection,
};
use crate::tracing::material::{lighting, Material};
use crate::tracing::point_light::PointLight;
use crate::tracing::ray::Ray;
use crate::tracing::sphere::Sphere;
use crate::tracing::shape::{Shape, WorldShape};

pub struct World {
    objects: Vec<WorldShape>,
    light_source: PointLight,
}

impl World {
    pub fn empty() -> World {
        World {
            objects: vec![],
            light_source: PointLight::black_light(),
        }
    }
    pub fn default() -> World {
        World::new(default_spheres(), PointLight::default())
    }

    pub fn new(objects: Vec<WorldShape>, light_source: PointLight) -> World {
        World { objects, light_source }
    }

    /**
     * Determine the Color given a PreComputedIntersection.
     */
    fn shade_hit(&self, pre_computations: PreComputedIntersection) -> Color {
        let in_shadow = self.is_shadowed(pre_computations.over_point);
        lighting(
            pre_computations.thing.material(),
            &self.light_source,
            pre_computations.point,
            pre_computations.eye_vector,
            pre_computations.normal_vector,
            in_shadow,
        )
    }

    /**
     * Calculate the color produced by firing ray at this World.
     */
    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = &self.intersected_by(ray);
        let hit = intersections.hit();
        match hit {
            Some(hit) => self.shade_hit(hit.pre_computations(ray)),
            None => Color::BLACK,
        }
    }

    pub fn intersected_by(&self, ray: &Ray) -> Intersections {
        let intersections: Vec<Intersections> =
            self.objects.iter().map(|it| it.intersect(ray)).collect();
        Intersections::combine(intersections)
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let point_to_light = self.light_source.position() - point;
        let distance = point_to_light.magnitude();
        let direction = point_to_light.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersected_by(&ray);

        let intersections_argument = &intersections;
        let hit = intersections_argument.hit();
        match hit {
            Some(hit) if hit.time() < distance => true,
            _ => false,
        }
    }
}

fn default_spheres() -> Vec<WorldShape> {
    let outer_sphere_material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
    let outer_sphere = Sphere::new(Point::origin(), outer_sphere_material);
    let inner_sphere = Sphere::new(Point::origin(), Material::default())
        .with_transform(scaling(0.5, 0.5, 0.5));

    let mut spheres: Vec<WorldShape> = Vec::new();
    spheres.push(Box::new(outer_sphere));
    spheres.push(Box::new(inner_sphere));
    spheres
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::transformations::{scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::helper::EPSILON;
    use crate::tracing::intersection::{Intersection};
    use crate::tracing::material::Material;
    use crate::tracing::point_light::PointLight;
    use crate::tracing::ray::Ray;
    use crate::tracing::sphere::Sphere;
    use crate::tracing::world::{default_spheres, World};
    use crate::tracing::shape::{Shape, WorldShape};
    use std::ops::Deref;

    #[test]
    fn creating_a_world() {
        let world = World::empty();
        assert!(world.objects.is_empty());
        assert_eq!(PointLight::black_light(), world.light_source);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = World::default();
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));

        let ray_argument = &ray;
        let intersections = world.intersected_by(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(4, intersections.len());
        assert_eq!(4.0, intersections[0].time());
        assert_eq!(4.5, intersections[1].time());
        assert_eq!(5.5, intersections[2].time());
        assert_eq!(6.0, intersections[3].time());
    }

    #[test]
    fn shading_an_intersection() {
        let world = World::default();
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = world.objects.first().unwrap().clone();
        let sphere = &shape;
        let ray_argument = &ray;
        let intersect = &sphere.intersect(ray_argument).intersections[0];

        let comps = intersect.pre_computations(&ray);

        let color = world.shade_hit(comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let light_source = PointLight::new(Point::at(0.0, 0.25, 0.0), Color::WHITE);
        let world = World::new(default_spheres(), light_source);
        let ray = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        let sphere = &world.objects[1];
        let ray_argument = &ray;
        let intersect = &sphere.intersect(ray_argument).intersections[1];

        let comps = intersect.pre_computations(&ray);

        let color = world.shade_hit(comps);
        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let world = World::default();
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 1, 0));

        let color = world.color_at(&ray);
        assert_eq!(Color::BLACK, color);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let world = World::default();
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));

        let color = world.color_at(&ray);
        assert_eq!(Color::new(0.38066, 0.47583, 0.28550), color);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let outer_sphere_material = Material::new(Color::new(0.8, 1.0, 0.6), 1.0, 0.7, 0.2, 200.0);
        let outer_sphere = Sphere::new(Point::origin(), outer_sphere_material);
        let material = Material::default().with_ambient(1.0);
        let inner_sphere = Sphere::new(Point::origin(), material)
            .with_transform(scaling(0.5, 0.5, 0.5));

        let world = World::new(
            vec![Box::new(outer_sphere.clone()), Box::new(inner_sphere.clone())],
            PointLight::default(),
        );

        let ray = Ray::new(Point::at(0.0, 0.0, 0.75), Vector::new(0, 0, -1));

        let color = world.color_at(&ray);
        assert_eq!(*inner_sphere.material().color(), color);
    }

    #[test]
    fn no_shadow_when_nothing_colinear_with_point_and_light() {
        let world = World::default();
        let point = Point::at(0, 10, 0);
        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn shadow_when_object_point_and_light() {
        let world = World::default();
        let point = Point::at(10, -10, 10);
        assert!(world.is_shadowed(point));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let world = World::default();
        let point = Point::at(-20, 20, -20);
        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn no_shadow_when_object_behind_poing() {
        let world = World::default();
        let point = Point::at(-2, 2, -2);
        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight::new(Point::at(0, 0, -10), Color::WHITE);

        let sphere_one = Box::new(Sphere::default());
        let sphere_two = Box::new(Sphere::default().with_transform(translation(0, 0, 10)));

        let mut objects: Vec<WorldShape> = Vec::new();
        objects.push(sphere_one);
        objects.push(sphere_two.clone());

        let world = World::new(objects, light);

        let ray = Ray::new(Point::at(0, 0, 5), Vector::new(0, 0, 1));
        let intersection = Intersection::new(4.0, sphere_two);

        let pre_computations = intersection.pre_computations(&ray);
        let color = world.shade_hit(pre_computations);
        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere: WorldShape = Box::new(Sphere::default().with_transform(transformations::translation(0, 0, 1)));
        let intersection = Intersection::new(5.0, sphere);
        let pre_computations = intersection.pre_computations(&ray);
        assert!(pre_computations.over_point.z < -EPSILON / 2.0);
        assert!(pre_computations.point.z > pre_computations.over_point.z);
    }
}
