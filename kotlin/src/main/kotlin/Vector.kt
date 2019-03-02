private val ZERO = Vector(0.0, 0.0, 0.0)

class Vector(val x: Double, val y: Double, val z: Double) {

    override fun equals(other: Any?) = when(other) {
        is Vector -> almost(x, other.x) && almost(y, other.y) && almost(z, other.z)
        else -> false
    }

    override fun toString() = "{x: $x, y: $y, z: $z}"

    fun plus(other: Vector) = Vector(x + other.x, y + other.y, z + other.z)
    fun minus(other: Vector) = Vector(x - other.x, y - other.y, z - other.z)
    fun negate() = ZERO.minus(this)
    fun times(scalar: Double) = Vector(x * scalar, y * scalar, z * scalar)
    fun dividedBy(scalar: Double) = times(1/scalar)
    fun magnitude() = Math.sqrt(x * x + y * y + z * z)
    fun normalize() = Vector(x / magnitude(), y / magnitude(), z / magnitude())
    fun dot(other: Vector) = x * other.x + y * other.y + z * other.z
    fun cross(b: Vector) = Vector(y * b.z - z * b.y, z * b.x - x * b.z, x * b.y - y * b.x)
}