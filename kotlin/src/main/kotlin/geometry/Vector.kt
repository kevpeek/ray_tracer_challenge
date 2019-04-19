package geometry

import helper.approximately

private val ZERO = Vector(0.0, 0.0, 0.0)

/**
 * Represents a vector in three dimensional space.
 */
class Vector(val x: Double, val y: Double, val z: Double) {

    constructor(x: Number, y: Number, z: Number) : this(x.toDouble(), y.toDouble(), z.toDouble())

    override fun equals(other: Any?) = when(other) {
        is Vector -> x approximately other.x && y approximately other.y && z approximately other.z
        else -> false
    }

    override fun toString() = "{x: $x, y: $y, z: $z}"

    operator fun plus(other: Vector) = Vector(x + other.x, y + other.y, z + other.z)
    operator fun minus(other: Vector) = Vector(x - other.x, y - other.y, z - other.z)
    operator fun unaryMinus() = ZERO - this
    operator fun times(scalar: Double) = Vector(x * scalar, y * scalar, z * scalar)
    operator fun div(scalar: Double) = times(1/scalar)
    fun magnitude() = Math.sqrt(x * x + y * y + z * z)
    fun normalize() = Vector(x / magnitude(), y / magnitude(), z / magnitude())
    fun dot(other: Vector) = x * other.x + y * other.y + z * other.z
    fun cross(b: Vector) = Vector(y * b.z - z * b.y, z * b.x - x * b.z, x * b.y - y * b.x)

    /**
     * By convention, a Vector is represented as a 4x1 matrix with a 4th value of 0.
     */
    fun asMatrix() = Matrix.ofSize(4, 1).of(x, y, z, 0)
}