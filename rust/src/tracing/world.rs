use std::ops::Deref;

use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::geometry::transformations::scaling;
use crate::tracing::intersection::{Intersections, PreComputedIntersection};
use crate::tracing::material::{Material};
use crate::tracing::point_light::PointLight;
use crate::tracing::ray::Ray;
use crate::tracing::shapes::shape::{Shape, WorldShape};
use crate::tracing::shapes::sphere::Sphere;

type BoxedShape = Box<dyn Shape>;

pub struct World {
    objects: Vec<BoxedShape>,
    light_source: PointLight,
    shadows_enabled: bool
}

impl World {
    pub fn empty() -> World {
        World::new(vec![], PointLight::black_light())
    }
    pub fn default() -> World {
        World::new(default_spheres(), PointLight::default())
    }

    pub fn new(objects: Vec<BoxedShape>, light_source: PointLight) -> World {
        World {
            objects,
            light_source,
            shadows_enabled: true,
        }
    }

    pub fn without_shadows(self) -> World {
        World { objects: self.objects, light_source: self.light_source, shadows_enabled: false }
    }

    pub fn plus_shape(self, new_shape: BoxedShape) -> World {
        let mut objects = self.objects;
        objects.push(new_shape);
        World::new(objects, self.light_source)
    }

    pub fn objects(&self) -> Vec<WorldShape> {
        self.objects.iter().map(Box::deref).collect()
    }

    /**
     * Calculate the color produced by firing ray at this World.
     */
    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_internal(ray, 5)
    }

    fn color_at_internal(&self, ray: &Ray, recursion_limit: usize) -> Color {
        let intersections = &self.intersected_by(ray);
        let hit = intersections.hit();
        match hit {
            Some(hit) => self.shade_hit(hit.pre_computations(ray), recursion_limit),
            None => Color::BLACK,
        }
    }

    /**
     * Determine the Color given a PreComputedIntersection.
     */
    fn shade_hit(&self, pre_computations: PreComputedIntersection, recursion_limit: usize) -> Color {
        let in_shadow = self.is_shadowed(pre_computations.over_point);
        let light = &self.light_source;
        // Using over_point here fixes fuzziness with checker pattern.
        let position = pre_computations.over_point;
        let eye_vector_argument = pre_computations.eye_vector;
        let normal = pre_computations.normal_vector;

        let surface_color = pre_computations.thing.lighting(light, position, eye_vector_argument, normal, in_shadow);
        let reflected_color = self.reflect_color(pre_computations, recursion_limit);
        surface_color + reflected_color
    }

    fn reflect_color(&self, pre_computations: PreComputedIntersection, recursion_limit: usize) -> Color {
        if recursion_limit == 0 {
            return Color::BLACK;
        }
        if pre_computations.thing.material().reflective() == 0.0 {
            return Color::BLACK;
        }

        let reflect_ray = Ray::new(pre_computations.over_point, pre_computations.reflect_vector);
        let color = self.color_at_internal(&reflect_ray, recursion_limit - 1);
        color * pre_computations.thing.material().reflective()
    }

    pub fn intersected_by(&self, ray: &Ray) -> Intersections {
        let intersections: Vec<Intersections> =
            self.objects().iter().map(|it| it.intersect(ray)).collect();
        Intersections::combine(intersections)
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        if !self.shadows_enabled {
            return false
        }

        let point_to_light = self.light_source.position() - point;
        let distance = point_to_light.magnitude();
        let direction = point_to_light.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersected_by(&ray);

        let intersections_argument = &intersections;
        let hit = intersections_argument.hit();

        matches!(hit, Some(hit) if hit.time() < distance)
    }
}

fn default_spheres() -> Vec<BoxedShape> {
    let outer_sphere_material = Material::solid_colored(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0);
    let outer_sphere = Sphere::new(Point::origin(), outer_sphere_material);
    let inner_sphere =
        Sphere::new(Point::origin(), Material::default()).with_transform(scaling(0.5, 0.5, 0.5));

    vec![Box::new(outer_sphere), Box::new(inner_sphere)]
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::transformations::{scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::helper::EPSILON;
    use crate::tracing::intersection::Intersection;
    use crate::tracing::material::Material;
    use crate::tracing::point_light::PointLight;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::sphere::Sphere;
    use crate::tracing::world::{default_spheres, BoxedShape, World};
    use crate::tracing::shapes::plane::Plane;

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

        let color = world.shade_hit(comps, 5);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let light_source = PointLight::new(Point::at(0.0, 0.25, 0.0), Color::WHITE);
        let world = World::new(default_spheres(), light_source);
        let ray = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        let sphere = &world.objects[1];
        let intersect = &sphere.intersect(&ray).intersections[1];

        let comps = intersect.pre_computations(&ray);

        let color = world.shade_hit(comps, 5);
        assert_eq!(Color::new(0.975222, 0.975222, 0.975222), color);
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
        let outer_sphere_material = Material::solid_colored(Color::new(0.8, 1.0, 0.6), 1.0, 0.7, 0.2, 200.0, 0.0);
        let outer_sphere = Sphere::new(Point::origin(), outer_sphere_material);
        let material = Material::default().with_ambient(1.0);
        let inner_sphere =
            Sphere::new(Point::origin(), material).with_transform(scaling(0.5, 0.5, 0.5));

        let world = World::new(
            vec![Box::new(outer_sphere), Box::new(inner_sphere)],
            PointLight::default(),
        );

        let ray = Ray::new(Point::at(0.0, 0.0, 0.75), Vector::new(0, 0, -1));

        let color = world.color_at(&ray);
        assert_eq!(Material::default().color(), color);
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
        let sphere_two = Sphere::default().with_transform(translation(0, 0, 10));

        let mut objects: Vec<BoxedShape> = Vec::new();
        objects.push(sphere_one);
        objects.push(Box::new(sphere_two.clone()));

        let world = World::new(objects, light);

        let ray = Ray::new(Point::at(0, 0, 5), Vector::new(0, 0, 1));
        let intersection = Intersection::new(4.0, &sphere_two);

        let pre_computations = intersection.pre_computations(&ray);
        let color = world.shade_hit(pre_computations, 5);
        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere: BoxedShape =
            Box::new(Sphere::default().with_transform(transformations::translation(0, 0, 1)));
        let intersection = Intersection::new(5.0, sphere.as_ref());
        let pre_computations = intersection.pre_computations(&ray);
        assert!(pre_computations.over_point.z < -EPSILON / 2.0);
        assert!(pre_computations.point.z > pre_computations.over_point.z);
    }

    #[test]
    fn reflected_color_of_non_reflective_surface() {
        let shape = Sphere::default().with_material(Material::default().with_ambient(1.0));
        let world = World::new(vec![Box::new(shape.clone())], PointLight::default());
        let ray = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        let intersection = Intersection::new(1.0, &shape);
        let pre_computations = intersection.pre_computations(&ray);

        assert_eq!(Color::BLACK, world.reflect_color(pre_computations, 5))
    }

    #[test]
    fn reflective_color_of_reflective_material() {
        let shape = Plane::new()
            .with_material(Material::default().with_reflective(0.5))
            .with_transform(transformations::translation(0, -1, 0));
        let world = World::default().plus_shape(Box::new(shape.clone()));
        let ray = Ray::new(Point::at(0, 0, -3), Vector::new(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        let intersection = Intersection::new(2.0_f64.sqrt(), &shape);
        let pre_computations = intersection.pre_computations(&ray);

        assert_eq!(Color::new(0.19033, 0.23791, 0.142749), world.reflect_color(pre_computations, 5));
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let shape = Plane::new()
            .with_material(Material::default().with_reflective(0.5))
            .with_transform(transformations::translation(0, -1, 0));
        let world = World::default().plus_shape(Box::new(shape.clone()));

        let ray = Ray::new(Point::at(0, 0, -3), Vector::new(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        let intersection = Intersection::new(2.0_f64.sqrt(), &shape);
        let pre_computations = intersection.pre_computations(&ray);

        assert_eq!(Color::new(0.84424, 0.89182, 0.79666), world.shade_hit(pre_computations, 5));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let light = PointLight::new(Point::origin(), Color::WHITE);

        // Placing the Ray inside a reflective sphere should produce infinite reflection if we don't stop it.
        let sphere = Sphere::default().with_material(Material::default().with_reflective(1.0));
        let world = World::new(vec![Box::new(sphere)], light);

        let ray = Ray::new(Point::origin(), Vector::new(0, 1, 0));

        // this should complete.
        let color = world.color_at(&ray);
    }
}
