package display

import helper.approximately
import kotlin.math.roundToInt

data class Color(val red: Double, val green: Double, val blue: Double) {

    constructor(red: Number, green: Number, blue: Number):this(red.toDouble(), green.toDouble(), blue.toDouble())

    companion object {
        val BLACK = Color(0.0, 0.0, 0.0)
        val RED = Color(1.0, 0.0, 0.0)
        val GREEN = Color(0.0, 1.0, 0.0)
        val BLUE = Color(0.0, 0.0, 1.0)
        val WHITE = Color(1.0, 1.0, 1.0)
    }

    override fun equals(other: Any?) = when(other) {
        is Color -> red approximately  other.red && green approximately  other.green && blue approximately other.blue
        else -> false
    }

    fun as255() = listOf(red, green, blue).map(this::valueTo255)
    private fun valueTo255(value: Double) = Math.min(255, Math.max(0, (value * 255).roundToInt()))

    operator fun plus(other: Color) = Color(red + other.red, green + other.green, blue + other.blue)
    operator fun minus(other: Color) = Color(red - other.red, green - other.green, blue - other.blue)
    operator fun times(scalar: Double) = Color(red * scalar, green * scalar, blue * scalar)
    operator fun times(other: Color) = Color(red * other.red, green * other.green, blue * other.blue)
}