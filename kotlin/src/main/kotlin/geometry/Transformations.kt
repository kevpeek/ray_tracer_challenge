package geometry

import kotlin.math.cos
import kotlin.math.sin

/**
 * Produce a transformation matrix that shifts by the supplied x, y, and z values.
 */
fun translation(x: Number, y: Number, z: Number) =
    Matrix.ofSize(4, 4).of(
        1, 0, 0, x,
        0, 1, 0, y,
        0, 0, 1, z,
        0, 0, 0, 1
    )

/**
 * A transformation that grows or shrinks an object.
 */
fun scaling(x: Number, y: Number, z: Number) =
    Matrix.ofSize(4, 4).of(
        x, 0, 0, 0,
        0, y, 0, 0,
        0, 0, z, 0,
        0, 0, 0, 1
    )

/**
 * Rotation around the x-axis.
 */
fun rotationX(radians: Double) =
    Matrix.ofSize(4, 4).of(
        1, 0, 0, 0,
        0, cos(radians), -sin(radians), 0,
        0, sin(radians), cos(radians), 0,
        0, 0, 0, 1
    )

/**
 * Rotation around the y-axis.
 */
fun rotationY(radians: Double) =
    Matrix.ofSize(4, 4).of(
        cos(radians), 0, sin(radians), 0,
        0, 1, 0, 0,
        -sin(radians), 0, cos(radians), 0,
        0, 0, 0, 1
    )

/**
 * Rotation around the z-axis.
 */
fun rotationZ(radians: Double) =
    Matrix.ofSize(4, 4).of(
        cos(radians), -sin(radians), 0, 0,
        sin(radians), cos(radians), 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    )

/**
 * Causes an object to....slant, I guess?
 */
fun shearing(xy: Number, xz: Number, yx: Number, yz: Number, zx: Number, zy: Number) =
    Matrix.ofSize(4, 4).of(
        1, xy, xz, 0,
        yx, 1, yz, 0,
        zx, zy, 1, 0,
        0, 0, 0, 1
    )

/**
 * Produces a transform to create a point of view looking from 'from' to 'to' with 'up' defining the
 * upward direction.
 */
fun viewTransform(from: Point, to: Point, up: Vector): Matrix {
    val forward = (to - from).normalize()
    val normalizedUp = up.normalize()
    val left = forward.cross(normalizedUp)
    val trueUp = left.cross(forward)

    val orientation = Matrix.square(4).of(
        left.x, left.y, left.z, 0,
        trueUp.x, trueUp.y, trueUp.z, 0,
        -(forward.x), -(forward.y), -(forward.z), 0,
        0, 0, 0, 1
    )

    return orientation * translation(-(from.x), -(from.y), -(from.z))
}
