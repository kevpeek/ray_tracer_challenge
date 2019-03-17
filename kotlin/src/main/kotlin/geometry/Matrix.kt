package geometry

import helper.approximately
import helper.times

class Matrix(private val height: Int, private val width: Int, private vararg val values: Double) {

    /**
     * Secondary constructor to streamline instantiation with non-decimal values.
     */
    constructor(rows: Int, columns: Int, vararg nums: Number) : this(rows, columns, *nums.map(Number::toDouble).toDoubleArray())

    companion object Factory {
        fun identity(size: Int): Matrix {
            val values = ((1..size) * (1..size)).map { (row, col) -> if (row == col) 1.0 else 0.0 }.toDoubleArray()
            return Matrix(4, 4, *values)
        }
    }


    override fun equals(other: Any?): Boolean = when(other) {
        is Matrix -> (values zip other.values).all { (a, b) -> a approximately b }
        else -> false
    }

    override fun toString(): String = values.joinToString()

    operator fun get(row: Int, column: Int) = values[getIndexFor(row, column)]

    operator fun times(other: Matrix): Matrix {
        val newValues = (0 until height).flatMap { rowIndex ->
            val row = getRow(rowIndex)
            (0 until other.width).map { columnIndex ->
                val column = other.getColumn(columnIndex)
                row.zip(column) { a, b -> a * b }.sum()
            }
        }.toDoubleArray()

        return Matrix(this.height, other.width, *newValues)
    }

    operator fun times(point: Point): Point {
        val result = this * Matrix(height, 1, point.x, point.y, point.z, 1)
        return Point(result[0, 0], result[1, 0], result[2, 0])
    }

    private fun getRow(rowIndex: Int) = (0 until width).map { columnIndex -> get(rowIndex, columnIndex) }
    private fun getColumn(columnIndex: Int) = (0 until height).map { rowIndex -> get(rowIndex, columnIndex) }
    private fun getIndexFor(row: Int, column: Int) = row * width + column
}
