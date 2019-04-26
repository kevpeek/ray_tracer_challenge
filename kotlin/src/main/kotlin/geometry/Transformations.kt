package geometry

/**
 * Produce a transformation matrix that shifts by the supplied x, y, and z values.
 */
fun translation(x: Int, y: Int, z: Int) =
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
            0, Math.cos(radians), -Math.sin(radians), 0,
            0, Math.sin(radians), Math.cos(radians), 0,
            0, 0, 0, 1
        )

/**
 * Rotation around the y-axis.
 */
fun rotationY(radians: Double) =
    Matrix.ofSize(4, 4).of(
        Math.cos(radians), 0, Math.sin(radians), 0,
        0, 1, 0, 0,
        -Math.sin(radians), 0, Math.cos(radians), 0,
        0, 0, 0, 1
    )

/**
 * Rotation around the z-axis.
 */
fun rotationZ(radians: Double) =
    Matrix.ofSize(4, 4).of(
        Math.cos(radians), -Math.sin(radians), 0, 0,
        Math.sin(radians), Math.cos(radians), 0, 0,
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