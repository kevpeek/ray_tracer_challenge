use crate::display::color::Color;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::transformations::scaling;
use crate::tracing::intersection::{hit, intersects, Intersection, PreComputedIntersection};
use crate::tracing::material::{lighting, Material};
use crate::tracing::point_light::PointLight;
use crate::tracing::ray::Ray;
use crate::tracing::sphere::Sphere;

pub struct World {
    objects: Vec<Sphere>,
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
        World {
            objects: default_spheres(),
            light_source: PointLight::default(),
        }
    }

    pub fn new(objects: Vec<Sphere>, light_source: PointLight) -> World {
        World {
            objects,
            light_source,
        }
    }

    /**
     * Determine the Color given a PreComputedIntersection.
     */
    fn shade_hit(&self, pre_computations: PreComputedIntersection) -> Color {
        lighting(
            pre_computations.thing.material(),
            &self.light_source,
            pre_computations.point,
            pre_computations.eye_vector,
            pre_computations.normal_vector,
        )
    }

    /**
     * Calculate the color produced by firing ray at this World.
     */
    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = &self.intersected_by(ray);
        let hit = hit(intersections);
        match hit {
            Some(hit) => self.shade_hit(hit.pre_computations(ray)),
            None => Color::BLACK,
        }
    }

    pub fn intersected_by(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|it| intersects(it, ray))
            .collect();
        intersections.sort_by(|a, b| a.time().partial_cmp(&b.time()).unwrap());
        intersections
    }
}

fn default_spheres() -> Vec<Sphere> {
    let outer_sphere_material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
    let outer_sphere = Sphere::new(Point::origin(), outer_sphere_material, Matrix::identity(4));
    let inner_sphere = Sphere::new(Point::origin(), Material::default(), scaling(0.5, 0.5, 0.5));
    vec![outer_sphere, inner_sphere]
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::transformations::scaling;
    use crate::geometry::vector::Vector;
    use crate::tracing::intersection::{intersects};
    use crate::tracing::material::Material;
    use crate::tracing::point_light::PointLight;
    use crate::tracing::ray::Ray;
    use crate::tracing::sphere::Sphere;
    use crate::tracing::world::{default_spheres, World};

    #[test]
    fn creating_a_world() {
        let world = World::empty();
        assert!(world.objects.is_empty());
        assert_eq!(PointLight::black_light(), world.light_source);
    }

    #[test]
    fn default_world() {
        let default_world = World::default();

        assert_eq!(PointLight::default(), default_world.light_source);
        assert_eq!(default_spheres(), default_world.objects);
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
        let intersect = &intersects(&shape, &ray)[0];

        let comps = intersect.pre_computations(&ray);

        let color = world.shade_hit(comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let light_source = PointLight::new(Point::at(0.0, 0.25, 0.0), Color::new(1, 1, 1));
        let world = World::new(default_spheres(), light_source);
        let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));
        let shape = world.objects[1].clone();
        let intersect = &intersects(&shape, &ray)[1];

        let comps = intersect.pre_computations(&ray);

        let color = world.shade_hit(comps);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), color);
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
        let outer_sphere = Sphere::new(Point::origin(), outer_sphere_material, Matrix::identity(4));
        let material = Material::default().with_ambient(1.0);
        let inner_sphere = Sphere::new(Point::origin(), material, scaling(0.5, 0.5, 0.5));

        let world = World::new(
            vec![outer_sphere.clone(), inner_sphere.clone()],
            PointLight::default(),
        );

        let ray = Ray::new(Point::at(0.0, 0.0, 0.75), Vector::new(0, 0, -1));

        let color = world.color_at(&ray);
        assert_eq!(*inner_sphere.material().color(), color);
    }
}
