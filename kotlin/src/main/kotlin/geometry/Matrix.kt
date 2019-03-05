package geometry

import helper.almost
import kotlin.math.roundToInt

class Matrix(private vararg val values: Double) {

    private val size = Math.sqrt(values.size.toDouble()).roundToInt()

    override fun equals(other: Any?): Boolean = when(other) {
        is Matrix -> (values zip other.values).all { (a, b) -> almost(a, b) }
        else -> false
    }

    operator fun get(row: Int, column: Int) = values[getIndexFor(row, column)]

    private fun getIndexFor(row: Int, column: Int) = row * size + column
}