class Point(val x: Double, val y: Double, val z: Double) {

    override fun equals(other: Any?) = when(other) {
        is Point -> almost(x, other.x) && almost(y, other.y) && almost(z, other.z)
        else -> false
    }

    override fun toString() = "{x: $x, y: $y, z: $z}"

    fun plus(vector: Vector) = Point(x + vector.x, y + vector.y, z + vector.z)
    fun minus(vector: Vector) = Point(x - vector.x, y - vector.y, z - vector.z)
    fun minus(other: Point) = Vector(x - other.x, y - other.y, z - other.z)
}