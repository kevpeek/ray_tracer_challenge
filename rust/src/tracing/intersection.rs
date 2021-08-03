use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::helper::EPSILON;
use crate::tracing::ray::Ray;
use crate::tracing::sphere::Sphere;

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
pub struct PreComputedIntersection {
    pub time: f64,
    pub thing: Sphere,
    pub inside: bool,
    pub point: Point,
    pub over_point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    time: f64,
    thing: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(time: f64, thing: &Sphere) -> Intersection {
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

        PreComputedIntersection {
            time: self.time,
            thing: self.thing.clone(),
            inside,
            point,
            over_point,
            eye_vector,
            normal_vector: actual_normal,
        }
    }
}

/**
 * Returns the list of Intersections between the ray and sphere.
 */
pub fn intersects<'a>(sphere: &'a Sphere, ray: &Ray) -> Intersections<'a> {
    let transformed_ray = ray.transform(sphere.transform().inverse());
    let sphere_to_ray = transformed_ray.origin() - sphere.origin();
    let a = transformed_ray.direction().dot(transformed_ray.direction());
    let b = 2.0 * transformed_ray.direction().dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return intersections![];
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    intersections![Intersection::new(t1, sphere), Intersection::new(t2, sphere)]
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{scaling, translation};
    use crate::geometry::vector::Vector;
    use crate::tracing::intersection::{intersects, Intersection};
    use crate::tracing::material::Material;
    use crate::tracing::ray::Ray;
    use crate::tracing::sphere::Sphere;

    #[test]
    fn a_ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = intersects(&sphere, &ray);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].time);
        assert_eq!(6.0, intersections[1].time);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(Point::at(0, 1, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = intersects(&sphere, &ray);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(5.0, intersections[0].time);
        assert_eq!(5.0, intersections[1].time);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::at(0, 2, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = intersects(&sphere, &ray);

        let intersections = intersections.intersections;
        assert!(intersections.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = intersects(&sphere, &ray);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(-1.0, intersections[0].time);
        assert_eq!(1.0, intersections[1].time);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let ray = Ray::new(Point::at(0, 0, 5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = intersects(&sphere, &ray);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(-6.0, intersections[0].time);
        assert_eq!(-4.0, intersections[1].time);
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Sphere::default();
        let intersection = Intersection::new(3.5, &sphere);

        assert_eq!(3.5, intersection.time);
        assert_eq!(Sphere::default(), *intersection.thing);
    }

    #[test]
    fn aggregating_intersections() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);

        let intersections = vec![i1.clone(), i2.clone()];

        assert_eq!(2, intersections.len());
        assert_eq!(i1, intersections[0]);
        assert_eq!(i2, intersections[1]);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));

        let sphere = Sphere::default();
        let intersections = intersects(&sphere, &ray);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(sphere, *intersections[0].thing);
        assert_eq!(sphere, *intersections[1].thing);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let intersections = intersections![i1.clone(), i2.clone()];

        let the_hit = intersections.hit();
        assert_eq!(i1, *the_hit.unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(1.0, &sphere);
        let intersections = intersections![i1.clone(), i2.clone()];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert_eq!(i2, *the_hit.unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(-2.0, &sphere);
        let i2 = Intersection::new(-1.0, &sphere);
        let intersections = intersections![i1, i2];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert!(the_hit.is_none())
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(5.0, &sphere);
        let i2 = Intersection::new(7.0, &sphere);
        let i3 = Intersection::new(-3.0, &sphere);
        let i4 = Intersection::new(2.0, &sphere);
        let intersections = intersections![i1.clone(), i2.clone(), i3.clone(), i4.clone()];

        let intersections_argument = &intersections;
        let the_hit = intersections_argument.hit();
        assert_eq!(i4, *the_hit.unwrap());
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::new(Point::origin(), Material::default(), scaling(2, 2, 2));

        let intersections = intersects(&sphere, &ray);

        let intersections = intersections.intersections;
        assert_eq!(2, intersections.len());
        assert_eq!(3.0, intersections[0].time);
        assert_eq!(7.0, intersections[1].time);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::new(Point::origin(), Material::default(), translation(5, 0, 0));

        let intersections = intersects(&sphere, &ray);
        let intersections = intersections.intersections;
        assert!(intersections.is_empty());
    }

    #[test]
    fn precomputing_state_of_an_intersection() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();
        let intersection = &intersects(&shape, &ray).intersections[0];

        let comps = intersection.pre_computations(&ray);

        assert_eq!(intersection.time, comps.time);
        assert_eq!(*intersection.thing, comps.thing);
        assert_eq!(Point::at(0, 0, -1), comps.point);
        assert_eq!(Vector::new(0, 0, -1), comps.eye_vector);
        assert_eq!(Vector::new(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let ray = Ray::new(Point::at(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();

        let intersect = &intersects(&shape, &ray).intersections[0];

        let comps = intersect.pre_computations(&ray);
        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = Ray::new(Point::at(0, 0, 0), Vector::new(0, 0, 1));
        let shape = Sphere::default();

        let intersect = &intersects(&shape, &ray).intersections[1];

        let comps = intersect.pre_computations(&ray);
        assert!(comps.inside);
        assert_eq!(Point::at(0, 0, 1), comps.point);
        assert_eq!(Vector::new(0, 0, -1), comps.eye_vector);
        assert_eq!(Vector::new(0, 0, -1), comps.normal_vector);
    }
}
