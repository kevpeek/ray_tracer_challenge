use crate::geometry::matrix::Matrix;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use num::NumCast;

/**
 * Produce a transformation matrix that shifts by the supplied x, y, and z letues.
 */
pub fn translation<T: NumCast>(x: T, y: T, z: T) -> Matrix {
    Matrix::of_size(4, 4).of(vec![
        1.0,
        0.0,
        0.0,
        x.to_f64().unwrap(),
        0.0,
        1.0,
        0.0,
        y.to_f64().unwrap(),
        0.0,
        0.0,
        1.0,
        z.to_f64().unwrap(),
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

/**
 * A transformation that grows or shrinks an object.
 */
pub fn scaling<T: NumCast>(x: T, y: T, z: T) -> Matrix {
    Matrix::of_size(4, 4).of(vec![
        x.to_f64().unwrap(),
        0.0,
        0.0,
        0.0,
        0.0,
        y.to_f64().unwrap(),
        0.0,
        0.0,
        0.0,
        0.0,
        z.to_f64().unwrap(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

/**
 * Rotation around the x-axis.
 */
pub fn rotation_x(radians: f64) -> Matrix {
    Matrix::of_size(4, 4).of(vec![
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        radians.cos(),
        -radians.sin(),
        0.0,
        0.0,
        radians.sin(),
        radians.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

/**
 * Rotation around the y-axis.
 */
pub fn rotation_y(radians: f64) -> Matrix {
    Matrix::of_size(4, 4).of(vec![
        radians.cos(),
        0.0,
        radians.sin(),
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -radians.sin(),
        0.0,
        radians.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

/**
 * Rotation around the z-axis.
 */
pub fn rotation_z(radians: f64) -> Matrix {
    Matrix::of_size(4, 4).of(vec![
        radians.cos(),
        -radians.sin(),
        0.0,
        0.0,
        radians.sin(),
        radians.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

/**
 * Causes an object to....slant, I guess?
 */
pub fn shearing<T: NumCast>(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Matrix {
    Matrix::of_size(4, 4).of(vec![
        1.0,
        xy.to_f64().unwrap(),
        xz.to_f64().unwrap(),
        0.0,
        yx.to_f64().unwrap(),
        1.0,
        yz.to_f64().unwrap(),
        0.0,
        zx.to_f64().unwrap(),
        zy.to_f64().unwrap(),
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

/**
 * Produces a transform to create a point of view looking from 'from' to 'to' with 'up' defining the
 * upward direction.
 */
pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalize();
    let normalized_up = up.normalize();
    let left = forward.cross(normalized_up);
    let true_up = left.cross(forward);

    let orientation = Matrix::square(4).of(vec![
        left.x,
        left.y,
        left.z,
        0.0,
        true_up.x,
        true_up.y,
        true_up.z,
        0.0,
        -(forward.x),
        -(forward.y),
        -(forward.z),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]);

    &orientation * &translation(-(from.x), -(from.y), -(from.z))
}

#[cfg(test)]
mod tests {
    use crate::geometry::matrix::Matrix;
    use crate::geometry::point::Point;
    use crate::geometry::transformations::{
        rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform,
    };
    use crate::geometry::vector::Vector;
    use std::f64::consts::PI;

    #[test]
    fn multiplying_a_translation_matrix() {
        let translation = translation(5, -3, 2);
        let point = Point::at(-3, 4, 5);

        assert_eq!(Point::at(2, 1, 7), &translation * point);
    }

    #[test]
    fn multiplying_by_inverse_of_translation() {
        let translation = translation(5, -3, 2);
        let inverse_translation = translation.inverse();

        let point = Point::at(-3, 4, 5);

        assert_eq!(Point::at(-8, 7, 3), &inverse_translation * point);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let translation = translation(5, -3, 2);
        let vector = Vector::new(-3, 4, 5);

        assert_eq!(vector, &translation * vector);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let scaling = scaling(2, 3, 4);
        let point = Point::at(-4, 6, 8);

        assert_eq!(Point::at(-8, 18, 32), &scaling * point);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let scaling = scaling(2, 3, 4);
        let vector = Vector::new(-4, 6, 8);

        assert_eq!(Vector::new(-8, 18, 32), &scaling * vector);
    }

    #[test]
    fn multiplying_by_the_inverse_of_scaling_matrix() {
        let scaling = scaling(2, 3, 4);
        let inverse_scaling = scaling.inverse();
        let vector = Vector::new(-4, 6, 8);

        assert_eq!(Vector::new(-2, 2, 2), &inverse_scaling * vector);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_letue() {
        let x_reflection = scaling(-1, 1, 1);
        let point = Point::at(2, 3, 4);

        assert_eq!(Point::at(-2, 3, 4), &x_reflection * point);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let point = Point::at(0, 1, 0);

        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            Point::at(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
            &half_quarter * point
        );
        assert_eq!(Point::at(0.0, 0.0, 1.0), &full_quarter * point);
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let point = Point::at(0, 1, 0);

        let half_quarter = rotation_x(PI / 4.0);
        let inverse = half_quarter.inverse();

        assert_eq!(
            Point::at(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
            &inverse * point
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let point = Point::at(0, 0, 1);

        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            Point::at(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0),
            &half_quarter * point
        );
        assert_eq!(Point::at(1, 0, 0), &full_quarter * point);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let point = Point::at(0, 1, 0);

        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            Point::at(-(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0, 0.0),
            &half_quarter * point
        );
        assert_eq!(Point::at(-1, 0, 0), &full_quarter * point);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let point = Point::at(2, 3, 4);

        let transformation = shearing(1, 0, 0, 0, 0, 0);

        assert_eq!(Point::at(5, 3, 4), &transformation * point);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let point = Point::at(2, 3, 4);

        let transformation = shearing(0, 1, 0, 0, 0, 0);

        assert_eq!(Point::at(6, 3, 4), &transformation * point);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let point = Point::at(2, 3, 4);

        let transformation = shearing(0, 0, 1, 0, 0, 0);

        assert_eq!(Point::at(2, 5, 4), &transformation * point);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let point = Point::at(2, 3, 4);

        let transformation = shearing(0, 0, 0, 1, 0, 0);

        assert_eq!(Point::at(2, 7, 4), &transformation * point);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let point = Point::at(2, 3, 4);

        let transformation = shearing(0, 0, 0, 0, 1, 0);

        assert_eq!(Point::at(2, 3, 6), &transformation * point);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let point = Point::at(2, 3, 4);

        let transformation = shearing(0, 0, 0, 0, 0, 1);

        assert_eq!(Point::at(2, 3, 7), &transformation * point);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = Point::at(1, 0, 1);

        let transform_a = rotation_x(PI / 2.0);
        let transform_b = scaling(5, 5, 5);
        let transform_c = translation(10, 5, 7);

        let point2 = &transform_a * point;
        assert_eq!(Point::at(1, -1, 0), point2);

        let point3 = &transform_b * point2;
        assert_eq!(Point::at(5, -5, 0), point3);

        let point4 = &transform_c * point3;
        assert_eq!(Point::at(15, 0, 7), point4);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let point = Point::at(1, 0, 1);

        let transform_a = rotation_x(PI / 2.0);
        let transform_b = scaling(5, 5, 5);
        let transform_c = translation(10, 5, 7);

        let combined_transform = &(&transform_c * &transform_b) * &transform_a;

        assert_eq!(Point::at(15, 0, 7), &combined_transform * point);
    }

    #[test]
    fn fluently_chained_transformations_must_be_applied_in_reverse_order() {
        let point = Point::at(1, 0, 1);

        let transform_a = rotation_x(PI / 2.0);
        let transform_b = scaling(5, 5, 5);
        let transform_c = translation(10, 5, 7);

        let combined_transform = transform_a.then(&transform_b).then(&transform_c);

        assert_eq!(Point::at(15, 0, 7), &combined_transform * point);
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = Point::at(0, 0, 0);
        let to = Point::at(0, 0, -1);
        let up = Vector::new(0, 1, 0);

        let result = view_transform(from, to, up);
        assert_eq!(Matrix::identity(4), result);
    }

    #[test]
    fn view_transformation_looking_in_positive_z() {
        let from = Point::at(0, 0, 0);
        let to = Point::at(0, 0, 1);
        let up = Vector::new(0, 1, 0);

        let result = view_transform(from, to, up);
        assert_eq!(scaling(-1, 1, -1), result);
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point::at(0, 0, 8);
        let to = Point::at(0, 0, 0);
        let up = Vector::new(0, 1, 0);

        let result = view_transform(from, to, up);
        assert_eq!(translation(0, 0, -8), result);
    }

    #[test]
    fn an_arbitrayr_new_transformation() {
        let from = Point::at(1, 3, 2);
        let to = Point::at(4, -2, 8);
        let up = Vector::new(1, 1, 0);

        let result = view_transform(from, to, up);

        let expected_result = Matrix::of_size(4, 4).of(vec![
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857,
            0.59761, -0.71714, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);

        assert_eq!(expected_result, result);
    }
}
