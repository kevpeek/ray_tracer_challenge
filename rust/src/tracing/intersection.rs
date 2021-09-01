use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::{almost, EPSILON};
use crate::tracing::ray::Ray;
use crate::tracing::shapes::shape::WorldShape;

#[derive(Debug, PartialEq)]
pub struct Intersections<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

/*
 * Macro for variadic intersections![] functionality
 */
#[macro_export]
macro_rules! intersections {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            $crate::tracing::intersection::Intersections { intersections: temp_vec }
        }
    };
}

impl<'a> Intersections<'a> {
    pub fn combine(others: Vec<Intersections>) -> Intersections {
        let mut values: Vec<Intersection> =
            others.into_iter().flat_map(|it| it.intersections).collect();
        values.sort_by(|a, b| a.time().partial_cmp(&b.time()).unwrap());
        Intersections {
            intersections: values,
        }
    }

    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        Intersections { intersections }
    }

    pub fn empty() -> Intersections<'a> {
        intersections!()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    /**
     * Finds the Intersection with the lowest, non-negative time value.
     */
    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections
            .iter()
            .filter(|it| it.time.is_sign_positive())
            .min_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
    }
}

/**
 * Precompute details about the intersection.
 */
pub struct PreComputedIntersection<'a> {
    pub time: f64,
    pub thing: WorldShape<'a>,
    pub inside: bool,
    pub point: Point,
    pub over_point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
    pub reflect_vector: Vector,
}

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    time: f64,
    thing: WorldShape<'a>,
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Intersection) -> bool {
        almost(self.time, other.time) && &self.thing == &other.thing
    }
}

impl<'a> Intersection<'a> {
    pub fn new(time: f64, thing: WorldShape) -> Intersection {
        Intersection { time, thing }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    /**
     * Calculate the PreComputed details.
     */
    pub fn pre_computations(&self, ray: &Ray) -> PreComputedIntersection {
        let point = ray.position(self.time);
        let eye_vector = -ray.direction();
        let normal_vector = self.thing.normal_at(point).normalize();

        let inside = normal_vector.dot(eye_vector) < 0.0;
        let actual_normal = if inside {
            -normal_vector
        } else {
            normal_vector
        };

        let over_point = point + normal_vector * EPSILON;

        let reflect_vector = ray.direction().reflect(normal_vector);

        PreComputedIntersection {
            time: self.time,
            thing: self.thing,
            inside,
            point,
            over_point,
            eye_vector,
            normal_vector: actual_normal,
            reflect_vector
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::tracing::intersection::Intersection;
    use crate::tracing::material::Material;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::shape::{Shape, WorldShape};
    use crate::tracing::shapes::sphere::Sphere;
    use crate::tracing::shapes::plane::Plane;
    use num::integer::Roots;

    #[test]
    fn a_ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let ray_argument = &ray;
        let intersections = sphere.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].time);
        assert_eq!(6.0, intersections[1].time);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(Point::at(0, 1, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(5.0, intersections[0].time);
        assert_eq!(5.0, intersections[1].time);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::at(0, 2, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert!(intersections.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(-1.0, intersections[0].time);
        assert_eq!(1.0, intersections[1].time);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let ray = Ray::new(Point::at(0, 0, 5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(-6.0, intersections[0].time);
        assert_eq!(-4.0, intersections[1].time);
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere: WorldShape = &Sphere::default();
        let intersection = Intersection::new(3.5, sphere);

        assert_eq!(3.5, intersection.time);
        assert_eq!(sphere, intersection.thing);
    }

    #[test]
    fn aggregating_intersections() {
        let sphere: WorldShape = &Sphere::default();
        let i1 = Intersection::new(1.0, sphere.clone());
        let i2 = Intersection::new(2.0, sphere);

        let intersections = vec![i1.clone(), i2.clone()];

        assert_eq!(2, intersections.len());
        assert_eq!(i1, intersections[0]);
        assert_eq!(i2, intersections[1]);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));

        let sphere: WorldShape = &Sphere::default();
        let ray_argument = &ray;
        let intersections = sphere.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(sphere, intersections[0].clone().thing);
        assert_eq!(sphere, intersections[1].clone().thing);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let sphere: WorldShape = &Sphere::default();
        let i1 = Intersection::new(1.0, sphere.clone());
        let i2 = Intersection::new(2.0, sphere);
        let intersections = intersections![i1.clone(), i2.clone()];

        let the_hit = intersections.hit();
        assert_eq!(i1, *the_hit.unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let sphere: WorldShape = &Sphere::default();
        let i1 = Intersection::new(-1.0, sphere.clone());
        let i2 = Intersection::new(1.0, sphere);
        let intersections = intersections![i1.clone(), i2.clone()];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert_eq!(i2, *the_hit.unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let sphere: WorldShape = &Sphere::default();
        let i1 = Intersection::new(-2.0, sphere.clone());
        let i2 = Intersection::new(-1.0, sphere);
        let intersections = intersections![i1, i2];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert!(the_hit.is_none())
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let sphere: WorldShape = &Sphere::default();
        let i1 = Intersection::new(5.0, sphere.clone());
        let i2 = Intersection::new(7.0, sphere.clone());
        let i3 = Intersection::new(-3.0, sphere.clone());
        let i4 = Intersection::new(2.0, sphere.clone());
        let intersections = intersections![i1.clone(), i2.clone(), i3.clone(), i4.clone()];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert_eq!(i4, *the_hit.unwrap());
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere =
            Sphere::new(Point::origin(), Material::default()).with_transform(scaling(2, 2, 2));

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(3.0, intersections[0].time);
        assert_eq!(7.0, intersections[1].time);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere =
            Sphere::new(Point::origin(), Material::default()).with_transform(translation(5, 0, 0));

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);
        let intersections = intersections.intersections;
        assert!(intersections.is_empty());
    }

    #[test]
    fn precomputing_state_of_an_intersection() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();
        let sphere = &shape;
        let ray_argument = &ray;
        let intersection = &sphere.intersect(ray_argument).intersections[0];

        let comps = intersection.pre_computations(&ray);

        assert_eq!(intersection.time, comps.time);
        assert!(intersection.thing == comps.thing);
        assert_eq!(Point::at(0, 0, -1), comps.point);
        assert_eq!(Vector::new(0, 0, -1), comps.eye_vector);
        assert_eq!(Vector::new(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();

        let sphere = &shape;
        let ray_argument = &ray;
        let intersect = &sphere.intersect(ray_argument).intersections[0];

        let comps = intersect.pre_computations(&ray);
        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));
        let shape = Sphere::default();

        let sphere = &shape;
        let ray_argument = &ray;
        let intersect = &sphere.intersect(ray_argument).intersections[1];

        let comps = intersect.pre_computations(&ray);
        assert!(comps.inside);
        assert_eq!(Point::at(0, 0, 1), comps.point);
        assert_eq!(Vector::new(0, 0, -1), comps.eye_vector);
        assert_eq!(Vector::new(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn precompute_reflective_vector() {
        let shape = Plane::new();
        let ray = Ray::new(Point::at(0, 1, -1), Vector::new(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        let intersection = Intersection::new(2.0_f64.sqrt(), &shape);

        let pre_computations = intersection.pre_computations(&ray);
        assert_eq!(Vector::new(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0), pre_computations.reflect_vector);
    }
}
