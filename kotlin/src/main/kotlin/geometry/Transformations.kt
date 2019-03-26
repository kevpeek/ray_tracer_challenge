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

fun scaling(x: Int, y: Int, z: Int) =
    Matrix.ofSize(4, 4).of(
        x, 0, 0, 0,
        0, y, 0, 0,
        0, 0, z, 0,
        0, 0, 0, 1
    )