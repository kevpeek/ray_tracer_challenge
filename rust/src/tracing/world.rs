use crate::display::color::Color;
use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::transformations::scaling;
use crate::tracing::intersection::{intersects, Intersection};
use crate::tracing::material::Material;
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
            objects: defaultSpheres(),
            light_source: PointLight::default(),
        }
    }

    pub fn new(objects: Vec<Sphere>, light_source: PointLight) -> World {
        World {
            objects,
            light_source,
        }
    }

    pub fn intersected_by(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|it| intersects(it.clone(), ray))
            .collect();
        // intersections.sort_by(|a, b| a.time.partial_cmp(*b.time));

        panic!("fix that sort");
        intersections
    }
}

fn defaultSpheres() -> Vec<Sphere> {
    let outerSphereMaterial = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
    let outerSphere = Sphere::new(Point::origin(), outerSphereMaterial, Matrix::identity(4));
    let innerSphere = Sphere::new(Point::origin(), Material::default(), scaling(0.5, 0.5, 0.5));
    vec![outerSphere, innerSphere]
}

#[cfg(test)]
mod tests {
    use crate::display::color::Color;
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::transformations::scaling;
    use crate::geometry::vector::Vector;
    use crate::tracing::material::Material;
    use crate::tracing::point_light::PointLight;
    use crate::tracing::ray::Ray;
    use crate::tracing::sphere::Sphere;
    use crate::tracing::world::{defaultSpheres, World};

    #[test]
    fn creating_a_world() {
        let world = World::empty();
        assert!(world.objects.is_empty());
        assert_eq!(PointLight::black_light(), world.light_source);
    }

    #[test]
    fn default_world() {
        let defaultWorld = World::default();

        assert_eq!(PointLight::default(), defaultWorld.light_source);
        assert_eq!(defaultSpheres(), defaultWorld.objects);
    }

    // #[test]
    // fn intersect_a_world_with_a_ray() {
    //     let world = World::default();
    //     let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
    //
    //     let intersections = intersectWorld(world, ray);
    //
    //     assert_eq!(4, intersections.size);
    //     assert_eq!(4.0, intersections[0].time);
    //     assert_eq!(4.5, intersections[1].time);
    //     assert_eq!(5.5, intersections[2].time);
    //     assert_eq!(6.0, intersections[3].time);
    // }

    // #[test]
    // fn shading_an_intersection() {
    //     let world = World::default();
    //     let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
    //     let shape = world.objects.first();
    //     let intersect = intersects(shape, ray)[0];
    //
    //     let comps = intersect.preComputations(ray);
    //
    //     let color = world.shadeHit(comps);
    //     assert_eq!(Color(0.38066, 0.47583, 0.2855), color);
    // }

    // #[test]
    // fn shading_an_intersection_from_the_inside() {
    //     let lightSource = PointLight::new(Point::at(0.0, 0.25, 0.0), Color::new(1, 1, 1));
    //     let world = World::new(defaultSpheres(), lightSource);
    //     let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));
    //     let shape = world.objects[1];
    //     let intersect = intersects(shape, ray)[1];
    //
    //     let comps = intersect.preComputations(ray);
    //
    //     let color = world.shadeHit(comps);
    //     assert_eq!(Color(0.90498, 0.90498, 0.90498), color);
    // }

    // #[test]
    // fn the_color_when_a_ray_misses() {
    //     let world = World::default();
    //     let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 1, 0));
    //
    //     let color = world.colorAt(ray);
    //     assert_eq!(Color::BLACK, color);
    // }

    // #[test]
    // fn the_color_when_a_ray_hits() {
    //     let world = World::default();
    //     let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
    //
    //     let color = world.colorAt(ray);
    //     assert_eq!(Color::new(0.38066, 0.47583, 0.28550), color);
    // }

    // #[test]
    // fn the_color_with_an_intersection_behind_the_ray() {
    //     let outerSphereMaterial = Material::new(Color::new(0.8, 1.0, 0.6), 1.0, 0.7, 0.2, 200.0);
    //     let outerSphere = Sphere::new(Point::origin(), outerSphereMaterial, Matrix::identity(4));
    //     let material = Material::default().with_ambient(1.0);
    //     let innerSphere = Sphere::new(Point::origin(), material,scaling(0.5, 0.5, 0.5));
    //
    //     let world = World(listOf(outerSphere, innerSphere), DEFAULT_LIGHT);
    //
    //     let ray = Ray(Point(0, 0, 0.75), Vector(0, 0, -1));
    //
    //     let color = world.colorAt(ray);
    //     assert_eq!(innerSphere.material.color, color);
    // }
}
