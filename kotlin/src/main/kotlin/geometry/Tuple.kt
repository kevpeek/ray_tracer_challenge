package geometry

import helper.approximately

private val ZERO_VECTOR = Tuple(0.0, 0.0, 0.0, 0.0)

class Tuple(val x: Double, val y: Double, val z: Double, val w: Double) {

    constructor(x: Number, y: Number, z: Number, w: Number): this(x.toDouble(), y.toDouble(), z.toDouble(), w.toDouble())

    companion object Factory {
        fun point(x: Double, y: Double, z: Double) = Tuple(x, y, z, 1.0)
        fun vector(x: Double, y: Double, z: Double) = Tuple(x, y, z, 0.0)
    }

    fun isPoint() = w == 1.0
    fun isVector() = w == 0.0

    override fun equals(other: Any?) = when(other) {
        is Tuple -> x approximately  other.x && y approximately  other.y &&
                z approximately other.z && w approximately other.w
        else -> false
    }

    override fun toString() = "{x: $x, y: $y, z: $z, w: $w}"

    fun add(vector: Tuple) = Tuple(x + vector.x, y + vector.y, z + vector.z, w + vector.w)
    fun minus(other: Tuple) = Tuple(x - other.x, y - other.y, z - other.z, w - other.w)
    fun negate() = ZERO_VECTOR.minus(this)
    fun times(scalar: Double) = Tuple(x * scalar, y * scalar, z * scalar, w * scalar)
    fun dividedBy(scalar: Double) = times(1 / scalar)
    fun magnitude() = Math.sqrt(x * x + y * y + z * z)
    fun normalize() = Tuple(x / magnitude(), y / magnitude(), z / magnitude(), w)
    fun dot(other: Tuple) = x * other.x + y * other.y + z * other.z + w * other.w
    fun cross(b: Tuple) =
        vector(y * b.z - z * b.y, z * b.x - x * b.z, x * b.y - y * b.x)
}