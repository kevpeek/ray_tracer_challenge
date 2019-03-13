package geometry

import helper.approximately
import kotlin.math.roundToInt

class Matrix(private vararg val values: Double) {

    /**
     * Secondary constructor to streamline instantiation with non-decimal values.
     */
    constructor(vararg nums: Number) : this(*nums.map(Number::toDouble).toDoubleArray())

    private val size = Math.sqrt(values.size.toDouble()).roundToInt()

    override fun equals(other: Any?): Boolean = when(other) {
        is Matrix -> (values zip other.values).all { (a, b) -> a approximately b }
        else -> false
    }

    override fun toString(): String = values.joinToString()

    operator fun get(row: Int, column: Int) = values[getIndexFor(row, column)]

    operator fun times(other: Matrix): Matrix {
        val newValues = (0 until size).flatMap { rowIndex ->
            val row = getRow(rowIndex)
            (0 until size).map {columnIndex ->
                val column = other.getColumn(columnIndex)
                row.zip(column) { a, b -> a * b }.sum()
            }
        }.toDoubleArray()

        return Matrix(*newValues)
    }

    private fun getRow(rowIndex: Int) = (0 until size).map { columnIndex -> get(rowIndex, columnIndex) }
    private fun getColumn(columnIndex: Int) = (0 until size).map { rowIndex -> get(rowIndex, columnIndex) }
    private fun getIndexFor(row: Int, column: Int) = row * size + column
}