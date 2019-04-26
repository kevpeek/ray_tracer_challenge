package geometry

import helper.approximately

val WORLD_ORIGIN = Point(0, 0, 0)

/**
 * Represents a point in three dimensional space.
 */
class Point(val x: Double, val y: Double, val z: Double) {

    constructor(x: Number, y: Number, z: Number) : this(x.toDouble(), y.toDouble(), z.toDouble())

    override fun equals(other: Any?) = when(other) {
        is Point -> x approximately other.x && y approximately other.y && z approximately other.z
        else -> false
    }

    override fun toString() = "{x: $x, y: $y, z: $z}"

    operator fun plus(vector: Vector) = Point(x + vector.x, y + vector.y, z + vector.z)
    operator fun minus(vector: Vector) = Point(x - vector.x, y - vector.y, z - vector.z)
    operator fun minus(other: Point) = Vector(x - other.x, y - other.y, z - other.z)

    /**
     * By convention, a Point is treated as a 4x1 Matrix with a 4th element of 1.
     */
    fun asMatrix() = Matrix.ofSize(4, 1).of(x, y, z, 1)
}