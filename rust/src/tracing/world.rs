use crate::display::color::Color;
use crate::geometry::point::Point;
use crate::geometry::transformations::scaling;
use crate::tracing::intersection::{Intersection, Intersections, PreComputedIntersection};
use crate::tracing::material::Material;
use crate::tracing::point_light::PointLight;
use crate::tracing::ray::Ray;
use crate::tracing::shapes::shape::{Shape, WorldShape};
use num::traits::real::Real;
use num::traits::Pow;

type BoxedShape = Shape;

pub struct World {
    objects: Vec<BoxedShape>,
    light_source: PointLight,
    shadows_enabled: bool,
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
        World {
            objects: self.objects,
            light_source: self.light_source,
            shadows_enabled: false,
        }
    }

    pub fn plus_shape(self, new_shape: BoxedShape) -> World {
        let mut objects = self.objects;
        objects.push(new_shape);
        World::new(objects, self.light_source)
    }

    pub fn objects(&self) -> Vec<WorldShape> {
        self.objects.iter().collect()
    }

    /**
     * Calculate the color produced by firing ray at this World.
     */
    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_internal(ray, 5)
    }

    fn color_at_internal(&self, ray: &Ray, recursion_remaining: usize) -> Color {
        let intersections = &self.intersected_by(ray);
        let hit = intersections.hit();
        match hit {
            Some(hit) => self.shade_hit(hit.pre_computations(ray, &intersections), recursion_remaining),
            None => Color::BLACK,
        }
    }

    /**
     * Determine the Color given a PreComputedIntersection.
     */
    fn shade_hit(
        &self,
        pre_computations: PreComputedIntersection,
        recursion_remaining: usize,
    ) -> Color {
        let in_shadow = self.is_shadowed(pre_computations.over_point());
        let light = &self.light_source;

        let surface_color = pre_computations.lighting(light, in_shadow);
        let reflected_color = self.reflect_color(&pre_computations, recursion_remaining);
        let refracted_color = self.refracted_color(&pre_computations, recursion_remaining);

        if pre_computations.is_reflective() && pre_computations.is_transparent() {
            let reflectance = pre_computations.schlick();
            return surface_color
                + reflected_color * reflectance
                + refracted_color * (1.0 - reflectance);
        }
        surface_color + reflected_color + refracted_color
    }

    fn reflect_color(
        &self,
        pre_computations: &PreComputedIntersection,
        recursion_remaining: usize,
    ) -> Color {
        if recursion_remaining == 0 {
            return Color::BLACK;
        }
        if !pre_computations.is_reflective() {
            return Color::BLACK;
        }

        let color = self.color_at_internal(&pre_computations.reflect_ray(), recursion_remaining - 1);
        pre_computations.scale_reflection(color)
    }

    fn refracted_color(
        &self,
        pre_computations: &PreComputedIntersection,
        recursion_remaining: usize,
    ) -> Color {
        if recursion_remaining == 0 {
            return Color::BLACK;
        }

        if !pre_computations.is_transparent() {
            return Color::BLACK;
        }

        if pre_computations.has_total_internal_reflection() {
            return Color::BLACK;
        }

        let refracted_ray = pre_computations.refracted_ray();
        let color = self.color_at_internal(&refracted_ray, recursion_remaining - 1);
        pre_computations.scale_refraction(color)
    }

    pub fn intersected_by(&self, ray: &Ray) -> Intersections {
        let intersections: Vec<Intersection> = self
            .objects()
            .iter()
            .flat_map(|it| it.intersect(ray).into_iter())
            .collect();
        Intersections::new(intersections)
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        if !self.shadows_enabled {
            return false;
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
    let outer_sphere_material = Material::solid_colored(
        Color::new(0.8, 1.0, 0.6),
        0.1,
        0.7,
        0.2,
        200.0,
        0.0,
        0.0,
        1.0,
    );
    let outer_sphere = Shape::sphere().with_material(outer_sphere_material);
    let inner_sphere = Shape::sphere().with_transform(scaling(0.5, 0.5, 0.5));

    vec![outer_sphere, inner_sphere]
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::transformations::{scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::tracing::intersection::{Intersection, Intersections};
    use crate::tracing::material::Material;
    use crate::tracing::point_light::PointLight;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::plane::Plane;
    use crate::tracing::shapes::shape::{Shape, ShapeGeometry};
    use crate::tracing::shapes::sphere::Sphere;
    use crate::tracing::test_helpers::TestPattern;
    use crate::tracing::world::{default_spheres, BoxedShape, World};

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
        let intersect = &sphere.intersect(&ray)[0];

        let comps = intersect.pre_computations(&ray, &Intersections::empty());

        let color = world.shade_hit(comps, 5);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let light_source = PointLight::new(Point::at(0.0, 0.25, 0.0), Color::WHITE);
        let world = World::new(default_spheres(), light_source);
        let ray = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        let sphere = &world.objects[1];
        let intersect = &sphere.intersect(&ray)[1];

        let comps = intersect.pre_computations(&ray, &Intersections::empty());

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
        let outer_sphere_material = Material::solid_colored(
            Color::new(0.8, 1.0, 0.6),
            1.0,
            0.7,
            0.2,
            200.0,
            0.0,
            0.0,
            1.0,
        );
        let outer_sphere = Shape::sphere().with_material(outer_sphere_material);
        let material = Material::default().with_ambient(1.0);
        let inner_sphere = Shape::sphere()
            .with_transform(scaling(0.5, 0.5, 0.5))
            .with_material(material);

        let world = World::new(vec![outer_sphere, inner_sphere], PointLight::default());

        let ray = Ray::new(Point::at(0.0, 0.0, 0.75), Vector::new(0, 0, -1));

        let color = world.color_at(&ray);
        assert_eq!(Color::WHITE, color);
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

        let sphere_one = Shape::sphere();
        let sphere_two = Shape::sphere().with_transform(translation(0, 0, 10));

        let objects: Vec<BoxedShape> = vec![sphere_one, sphere_two.clone()];
        let world = World::new(objects, light);

        let ray = Ray::new(Point::at(0, 0, 5), Vector::new(0, 0, 1));
        let intersection = Intersection::new(4.0, &sphere_two);

        let pre_computations = intersection.pre_computations(&ray, &Intersections::empty());
        let color = world.shade_hit(pre_computations, 5);
        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
    }

    #[test]
    fn reflected_color_of_non_reflective_surface() {
        let shape = Shape::sphere().with_material(Material::default().with_ambient(1.0));
        let world = World::new(vec![shape.clone()], PointLight::default());
        let ray = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        let intersection = Intersection::new(1.0, &shape);
        let pre_computations = intersection.pre_computations(&ray, &Intersections::empty());

        assert_eq!(Color::BLACK, world.reflect_color(&pre_computations, 5))
    }

    #[test]
    fn reflective_color_of_reflective_material() {
        let shape = Shape::plane()
            .with_transform(transformations::translation(0, -1, 0))
            .with_material(Material::default().with_reflective(0.5));
        let world = World::default().plus_shape(shape.clone());
        let ray = Ray::new(
            Point::at(0, 0, -3),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let intersection = Intersection::new(2.0_f64.sqrt(), &shape);
        let pre_computations = intersection.pre_computations(&ray, &Intersections::empty());

        assert_eq!(
            Color::new(0.19033, 0.23791, 0.142749),
            world.reflect_color(&pre_computations, 5)
        );
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let shape = Shape::plane()
            .with_transform(transformations::translation(0, -1, 0))
            .with_material(Material::default().with_reflective(0.5));
        let world = World::default().plus_shape(shape.clone());

        let ray = Ray::new(
            Point::at(0, 0, -3),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let intersection = Intersection::new(2.0_f64.sqrt(), &shape);
        let pre_computations = intersection.pre_computations(&ray, &Intersections::empty());

        assert_eq!(
            Color::new(0.84424, 0.89182, 0.79666),
            world.shade_hit(pre_computations, 5)
        );
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let light = PointLight::new(Point::origin(), Color::WHITE);

        // Placing the Ray inside a reflective sphere should produce infinite reflection if we don't stop it.
        let sphere = Shape::sphere().with_material(Material::default().with_reflective(1.0));
        let world = World::new(vec![sphere], light);

        let ray = Ray::new(Point::origin(), Vector::new(0, 1, 0));

        // this should complete.
        let color = world.color_at(&ray);
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let world = World::default();
        let shape = world.objects.first().unwrap();
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let intersections = Intersections::new(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);

        let details = intersections[0].pre_computations(&ray, &intersections);
        assert_eq!(Color::BLACK, world.refracted_color(&details, 5));
    }

    #[test]
    fn refracted_color_at_max_recursion_depth() {
        let shape = Sphere::new().into_shape().with_material(
            Material::default()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );

        let world = World::empty().plus_shape(shape.clone());
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let intersections = Intersections::new(vec![
            Intersection::new(4.0, &shape),
            Intersection::new(6.0, &shape),
        ]);

        let details = intersections[0].pre_computations(&ray, &intersections);
        assert_eq!(Color::BLACK, world.refracted_color(&details, 0));
    }

    #[test]
    fn refraction_with_total_internal_reflection() {
        let outer_sphere = Sphere::new().into_shape().with_material(
            Material::default()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );
        let inner_sphere = Sphere::new()
            .into_shape()
            .with_transform(transformations::scaling(0.25, 0.25, 0.25));
        let world = World::new(
            vec![outer_sphere.clone(), inner_sphere.clone()],
            PointLight::default(),
        );

        let ray = Ray::new(
            Point::at(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new(0, 1, 0),
        );
        let intersections = Intersections::new(vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, &outer_sphere),
            Intersection::new(2.0_f64.sqrt() / 2.0, &outer_sphere),
        ]);
        let details = intersections[1].pre_computations(&ray, &intersections);
        assert_eq!(Color::BLACK, world.refracted_color(&details, 5));
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let outer_sphere = Sphere::new().into_shape().with_material(
            Material::default()
                .with_ambient(1.0)
                .with_pattern(TestPattern {}.without_transform()),
        );
        let inner_sphere = Sphere::new()
            .into_shape()
            .with_material(
                Material::default()
                    .with_transparency(1.0)
                    .with_refractive_index(1.5),
            )
            .with_transform(transformations::scaling(0.25, 0.25, 0.25));
        let world = World::new(
            vec![outer_sphere.clone(), inner_sphere.clone()],
            PointLight::default(),
        );

        let ray = Ray::new(Point::at(0.0, 0.0, 0.1), Vector::new(0, 1, 0));
        let intersections = Intersections::new(vec![
            Intersection::new(-0.9899, &outer_sphere),
            Intersection::new(-0.4899, &inner_sphere),
            Intersection::new(0.4899, &inner_sphere),
            Intersection::new(0.9899, &outer_sphere),
        ]);

        let details = intersections[2].pre_computations(&ray, &intersections);
        assert_eq!(
            Color::new(0.0, 0.99887, 0.04721),
            world.refracted_color(&details, 5)
        )
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let floor = Plane::new()
            .into_shape()
            .with_transform(transformations::translation(0, -1, 0))
            .with_material(
                Material::default()
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            );
        let ball = Sphere::new()
            .into_shape()
            .with_material(
                Material::default()
                    .with_color(Color::new(1, 0, 0))
                    .with_ambient(0.5),
            )
            .with_transform(transformations::translation(0.0, -3.5, -0.5));
        let world = World::default()
            .plus_shape(floor.clone())
            .plus_shape(ball.clone());

        let ray = Ray::new(
            Point::at(0, 0, -3),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let intersections = Intersections::new(vec![Intersection::new(2_f64.sqrt(), &floor)]);

        let details = intersections[0].pre_computations(&ray, &intersections);
        assert_eq!(
            Color::new(0.90391, 0.65391, 0.65391),
            world.shade_hit(details, 5)
        );
    }

    #[test]
    fn shade_hit_with_transparent_and_reflective_material() {
        let floor = Plane::new()
            .into_shape()
            .with_transform(transformations::translation(0, -1, 0))
            .with_material(
                Material::default()
                    .with_reflective(0.5)
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            );
        let ball = Sphere::new()
            .into_shape()
            .with_material(
                Material::default()
                    .with_color(Color::new(1, 0, 0))
                    .with_ambient(0.5),
            )
            .with_transform(transformations::translation(0.0, -3.5, -0.5));
        let world = World::default()
            .plus_shape(floor.clone())
            .plus_shape(ball.clone());

        let ray = Ray::new(
            Point::at(0, 0, -3),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let intersections = Intersections::new(vec![Intersection::new(2_f64.sqrt(), &floor)]);

        let details = intersections[0].pre_computations(&ray, &intersections);
        assert_eq!(
            Color::new(0.9014, 0.66392, 0.65991),
            world.shade_hit(details, 5)
        );
    }
}
