use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::{almost, EPSILON};
use crate::tracing::ray::Ray;
use crate::tracing::shapes::shape::{WorldShape, Shape};
use crate::display::color::Color;
use crate::tracing::point_light::PointLight;
use std::ops::Index;
use std::vec::IntoIter;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
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
    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        let mut intersections = intersections;
        intersections.sort_by(|a, b| {
            a.time().partial_cmp(&b.time()).unwrap()
        });
        Intersections { intersections }
    }

    pub fn empty() -> Intersections<'a> {
        Self::new(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn iter(&self) -> Iter<'_, Intersection<'a>> {
        self.intersections.iter()
    }

    pub fn into_iter(self) -> IntoIter<Intersection<'a>> {
        self.intersections.into_iter()
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

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.intersections[i]
    }
}

/**
 * Precompute details about the intersection.
 */
pub struct PreComputedIntersection<'a> {
    time: f64,
    thing: WorldShape<'a>,
    inside: bool,
    point: Point,
    over_point: Point,
    under_point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    reflect_vector: Vector,
    n1: f64,
    n2: f64,
}

impl<'a> PreComputedIntersection<'a> {
    pub fn is_reflective(&self) -> bool {
        self.thing.material().reflective() > 0.0
    }

    pub fn scale_reflection(&self, color: Color) -> Color {
        color * self.thing.material().reflective()
    }

    pub fn reflect_ray(&self) -> Ray {
        Ray::new(self.over_point, self.reflect_vector)
    }

    pub fn is_opaque(&self) -> bool {
        self.thing.material().transparency() == 0.0
    }

    pub fn scale_refraction(&self, color: Color) -> Color {
        color * self.thing.material().transparency()
    }

    pub fn lighting(&self, light: &PointLight, in_shadow: bool) -> Color {
        self.thing.lighting(light, self.over_point, self.eye_vector, self.normal_vector, in_shadow)
    }

    pub fn over_point(&self) -> Point {
        self.over_point
    }

    pub fn under_point(&self) -> Point {
        self.under_point
    }

    pub fn n1(&self) -> f64 {
        self.n1
    }

    pub fn n2(&self) -> f64 {
        self.n2
    }

    pub fn eye_vector(&self) -> &Vector {
        &self.eye_vector
    }

    pub fn normal(&self) -> &Vector {
        &self.normal_vector
    }
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
    pub fn pre_computations(&self, ray: &Ray, intersections: &Intersections) -> PreComputedIntersection {
        let point = ray.position(self.time);
        let eye_vector = -ray.direction();
        let normal_vector = self.thing.normal_at(point).normalize();

        let inside = normal_vector.dot(eye_vector) < 0.0;
        let normal_vector = if inside {
            -normal_vector
        } else {
            normal_vector
        };

        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;

        let reflect_vector = ray.direction().reflect(normal_vector);

        let (n1, n2) = self.find_refractive_indexes(intersections);

        PreComputedIntersection {
            time: self.time,
            thing: self.thing,
            inside,
            point,
            over_point,
            under_point,
            eye_vector,
            normal_vector,
            reflect_vector,
            n1,
            n2,
        }
    }

    /**
    * Find the refractive index on each side of the intersection.
    */
    fn find_refractive_indexes(&self, intersections: &Intersections) -> (f64, f64) {
        let mut containers: Vec<&Shape> = Vec::new();
        let mut n1 = 0.0;
        let mut n2 = 0.0;

        for intersection in intersections.iter() {
            if intersection == self {
                n1 = containers.last().map_or(1.0, |it| it.material().refractive_index());
            }

            if containers.contains(&intersection.thing) {
                let remove_index = containers.iter().position(|it| it == &intersection.thing).unwrap();
                containers.remove(remove_index);
            } else {
                containers.push(intersection.thing);
            }

            if intersection == self {
                n2 = containers.last().map_or(1.0, |it| it.material().refractive_index());
                break;
            }
        }

        (n1, n2)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::tracing::intersection::{Intersection, Intersections};
    use crate::tracing::material::Material;
    use crate::tracing::ray::Ray;
    use crate::tracing::shapes::shape::{WorldShape, Shape, ShapeGeometry};
    use crate::tracing::shapes::sphere::Sphere;
    use crate::tracing::shapes::plane::Plane;
    use num::integer::Roots;
    use crate::geometry::transformations;
    use crate::helper::EPSILON;

    #[test]
    fn a_ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::new();

        let ray_argument = &ray;
        let intersections = sphere.intersect(ray_argument);

        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0]);
        assert_eq!(6.0, intersections[1]);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(Point::at(0, 1, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::new();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        assert_eq!(2, intersections.len());
        assert_eq!(5.0, intersections[0]);
        assert_eq!(5.0, intersections[1]);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::at(0, 2, -5), Vector::new(0, 0, 1));
        let sphere = Shape::sphere();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        assert!(intersections.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));
        let sphere = Sphere::new();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        assert_eq!(2, intersections.len());
        assert_eq!(-1.0, intersections[0]);
        assert_eq!(1.0, intersections[1]);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let ray = Ray::new(Point::at(0, 0, 5), Vector::new(0, 0, 1));
        let sphere = Sphere::new();

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);

        assert_eq!(2, intersections.len());
        assert_eq!(-6.0, intersections[0]);
        assert_eq!(-4.0, intersections[1]);
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere: WorldShape = &Shape::sphere();
        let intersection = Intersection::new(3.5, sphere);

        assert_eq!(3.5, intersection.time);
        assert_eq!(sphere, intersection.thing);
    }

    #[test]
    fn aggregating_intersections() {
        let sphere: WorldShape = &Shape::sphere();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, sphere);

        let intersections = Intersections::new(vec![i1.clone(), i2.clone()]);

        assert_eq!(2, intersections.len());
        assert_eq!(i1, intersections[0]);
        assert_eq!(i2, intersections[1]);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));

        let sphere: WorldShape = &Shape::sphere();
        let ray_argument = &ray;
        let intersections = sphere.intersect(ray_argument);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(sphere, intersections[0].clone().thing);
        assert_eq!(sphere, intersections[1].clone().thing);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let sphere: WorldShape = &Shape::sphere();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let intersections = intersections![i1.clone(), i2.clone()];

        let the_hit = intersections.hit();
        assert_eq!(i1, *the_hit.unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let sphere: WorldShape = &Shape::sphere();
        let i1 = Intersection::new(-1.0, sphere);
        let i2 = Intersection::new(1.0, sphere);
        let intersections = intersections![i1.clone(), i2.clone()];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert_eq!(i2, *the_hit.unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let sphere: WorldShape = &Shape::sphere();
        let i1 = Intersection::new(-2.0, sphere);
        let i2 = Intersection::new(-1.0, sphere);
        let intersections = intersections![i1, i2];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert!(the_hit.is_none())
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let sphere: WorldShape = &Shape::sphere();
        let i1 = Intersection::new(5.0, sphere);
        let i2 = Intersection::new(7.0, sphere);
        let i3 = Intersection::new(-3.0, sphere);
        let i4 = Intersection::new(2.0, sphere);
        let intersections = intersections![i1.clone(), i2.clone(), i3.clone(), i4.clone()];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert_eq!(i4, *the_hit.unwrap());
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere =
            Shape::sphere().with_transform(scaling(2, 2, 2));

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
            Shape::sphere().with_transform(translation(5, 0, 0));

        let sphere_argument = &sphere;
        let ray_argument = &ray;
        let intersections = sphere_argument.intersect(ray_argument);
        let intersections = intersections.intersections;
        assert!(intersections.is_empty());
    }

    #[test]
    fn precomputing_state_of_an_intersection() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Shape::sphere();
        let intersection = &shape.intersect(&ray).intersections[0];

        let comps = intersection.pre_computations(&ray, &Intersections::empty());

        assert_eq!(intersection.time, comps.time);
        assert!(intersection.thing == comps.thing);
        assert_eq!(Point::at(0, 0, -1), comps.point);
        assert_eq!(Vector::new(0, 0, -1), comps.eye_vector);
        assert_eq!(Vector::new(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Shape::sphere();

        let intersect = &&shape.intersect(&ray).intersections[0];

        let comps = intersect.pre_computations(&ray, &Intersections::empty());
        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));

        let sphere = &Shape::sphere();
        let intersect = &sphere.intersect(&ray).intersections[1];

        let comps = intersect.pre_computations(&ray, &Intersections::empty());
        assert!(comps.inside);
        assert_eq!(Point::at(0, 0, 1), comps.point);
        assert_eq!(Vector::new(0, 0, -1), comps.eye_vector);
        assert_eq!(Vector::new(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn precompute_reflective_vector() {
        let shape = Shape::plane();
        let ray = Ray::new(Point::at(0, 1, -1), Vector::new(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        let intersection = Intersection::new(2.0_f64.sqrt(), &shape);

        let pre_computations = intersection.pre_computations(&ray, &Intersections::empty());
        assert_eq!(Vector::new(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0), pre_computations.reflect_vector);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere =
            Shape::sphere().with_transform(transformations::translation(0, 0, 1));
        let intersection = Intersection::new(5.0, &sphere);
        let pre_computations = intersection.pre_computations(&ray, &Intersections::empty());
        assert!(pre_computations.over_point.z < -EPSILON / 2.0);
        assert!(pre_computations.point.z > pre_computations.over_point.z);
    }

    #[test]
    fn under_point_is_below_surface() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::new().into_shape().with_transform(transformations::translation(0, 0, 1));
        let intersection = Intersection::new(5.0, &shape);
        let intersections = Intersections::new(vec![intersection.clone()]);
        let details = intersection.pre_computations(&ray, &intersections);
        assert!(details.under_point.z > EPSILON / 2.0);
        assert!(details.point.z < details.under_point.z);
    }
}
