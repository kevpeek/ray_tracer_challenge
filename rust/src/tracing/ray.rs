use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use num::NumCast;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    /**
     * Calculates the position of the Ray after the specified amount of time.
     */
    pub fn position<T: NumCast>(&self, time: T) -> Point {
        self.origin + (self.direction * time.to_f64().unwrap())
    }

    /**
     * Return the Ray obtained by applying the supplied transformation to this Ray.
     */
    pub fn transform(&self, transformation: Matrix) -> Ray {
        Ray::new(
            &transformation * self.origin,
            &transformation * self.direction,
        )
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }
    pub fn origin(&self) -> Point {
        self.origin
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;
    use crate::geometry::transformations;
    use crate::geometry::transformations::translation;
    use crate::geometry::vector::Vector;
    use crate::tracing::ray::Ray;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point::at(1, 2, 3);
        let direction = Vector::new(4, 5, 6);

        let ray = Ray::new(origin, direction);

        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let ray = Ray::new(Point::at(2, 3, 4), Vector::new(1, 0, 0));

        assert_eq!(Point::at(2, 3, 4), ray.position(0));
        assert_eq!(Point::at(3, 3, 4), ray.position(1));
        assert_eq!(Point::at(1, 3, 4), ray.position(-1));
        assert_eq!(Point::at(4.5, 3.0, 4.0), ray.position(2.5));
    }

    #[test]
    fn translating_a_ray() {
        let ray = Ray::new(Point::at(1, 2, 3), Vector::new(0, 1, 0));
        let matrix = translation(3, 4, 5);

        let ray2 = ray.transform(matrix);
        assert_eq!(Point::at(4, 6, 8), ray2.origin);
        assert_eq!(Vector::new(0, 1, 0), ray2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let ray = Ray::new(Point::at(1, 2, 3), Vector::new(0, 1, 0));
        let matrix = transformations::scaling(2, 3, 4);

        let ray2 = ray.transform(matrix);
        assert_eq!(Point::at(2, 6, 12), ray2.origin);
        assert_eq!(Vector::new(0, 3, 0), ray2.direction);
    }
}
