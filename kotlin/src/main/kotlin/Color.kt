import kotlin.math.roundToInt

data class Color(val red: Double, val green: Double, val blue: Double) {

    companion object {
        val BLACK = Color(0.0, 0.0, 0.0)
        val RED = Color(1.0, 0.0, 0.0)
        val GREEN = Color(0.0, 1.0, 0.0)
        val BLUE = Color(0.0, 0.0, 1.0)
        val WHITE = Color(1.0, 1.0, 1.0)
    }

    override fun equals(other: Any?) = when(other) {
        is Color -> almost(red, other.red) && almost(green, other.green) && almost(blue, other.blue)
        else -> false
    }

    fun as255() = listOf(red, green, blue).map(this::valueTo255)
    private fun valueTo255(value: Double) = Math.min(255, Math.max(0, (value * 255).roundToInt()))

    fun plus(other: Color) = Color(red + other.red, green + other.green, blue + other.blue)
    fun minus(other: Color) = Color(red - other.red, green - other.green, blue - other.blue)
    fun times(scalar: Double) = Color(red * scalar, green * scalar, blue * scalar)
    fun times(other: Color) = Color(red * other.red, green * other.green, blue * other.blue)
}