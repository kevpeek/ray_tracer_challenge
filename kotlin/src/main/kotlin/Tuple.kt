class Tuple(val x: Double, val y: Double, val z: Double, val w: Double) {

    companion object Factory {
        fun point(x: Double, y: Double, z: Double) = Tuple(x, y, z, 1.0)
        fun vector(x: Double, y: Double, z: Double) = Tuple(x, y, z, 0.0)
    }

    private val EPSILON = .00001

    fun isPoint() = w == 1.0
    fun isVector() = w == 0.0

    override fun equals(other: Any?) = when(other) {
        is Tuple -> almost(x, other.x) && almost(y, other.y) && almost(z, other.z) && almost(w, other.w)
        else -> false
    }

    private fun almost(a: Double, b: Double) = Math.abs(a - b) < EPSILON

    fun add(vector: Tuple) = Tuple(x + vector.x, y + vector.y, z + vector.z, w + vector.w)
}