package geometry

import helper.approximately
import helper.times
import java.lang.RuntimeException

/**
 * Representation of a Matrix.
 *
 * Some operations are only supported for square matrices.
 */
class Matrix(private val height: Int, private val width: Int, private val values: List<Double>) {

    /**
     * Functions for creating Matrices.
     */
    companion object Factory {
        /**
         * For creating general Matrices.
         *
         * Matrix.ofSize(r, c).of(1, 2, 3, ....)
         */
        fun ofSize(rows: Int, columns: Int) = MatrixBuilder(rows, columns)

        /**
         * Creates an identity matrix of the specified size.
         *
         * Identity matrix: all values are zero, except for the top-left to bottom-right diagonal values
         * which are all 1.
         */
        fun identity(size: Int): Matrix {
            val values = ((1..size) * (1..size)).map { (row, col) -> if (row == col) 1.0 else 0.0 }
            return Matrix(4, 4, values)
        }
    }

    /**
     * Two Matrices are equal if all corresponding values are equal.
     */
    override fun equals(other: Any?): Boolean = when (other) {
        is Matrix -> (values zip other.values).all { (a, b) -> a approximately b }
        else -> false
    }

    override fun toString(): String = values.joinToString()

    operator fun get(row: Int, column: Int) = values[getIndexFor(row, column)]

    /**
     * Returns the values from the specified row.
     */
    private fun getRow(rowIndex: Int) = (0 until width).map { columnIndex -> get(rowIndex, columnIndex) }

    operator fun times(other: Matrix): Matrix {
        val newValues = (0 until height).flatMap { rowIndex ->
            val row = getRow(rowIndex)
            (0 until other.width).map { columnIndex ->
                val column = other.getColumn(columnIndex)
                row.zip(column) { a, b -> a * b }.sum()
            }
        }

        return Matrix(this.height, other.width, newValues)
    }

    operator fun times(point: Point): Point {
        val result = this * point.asMatrix()
        return Point(result[0, 0], result[1, 0], result[2, 0])
    }

    operator fun times(vector: Vector): Vector {
        val result = this * vector.asMatrix()
        return Vector(result[0, 0], result[1, 0], result[2, 0])
    }

    /**
     * Returns the transpose matrix, where each row of the original is a column of the transpose.
     */
    fun transpose() = Matrix(width, height, (0 until width).flatMap(this::getColumn))

    fun determinant(): Double = when {
        height == 2 && width == 2 -> this[0, 0] * this[1, 1] - this[0, 1] * this[1, 0]
        else -> getRow(0).mapIndexed { column, value -> cofactor(0, column) * value }.sum()
    }

    /**
     * Returns the sub matrix created by removing the specified row and column.
     */
    fun submatrix(row: Int, column: Int): Matrix {
        val indexesToKeep = ((0 until height) * (0 until width)).filter { (r, c) -> r != row && c != column }
        val valuesToKeep = indexesToKeep.map { (r, c) -> this[r, c] }
        return Matrix(height - 1, width - 1, valuesToKeep)
    }

    fun minor(row: Int, column: Int): Double = submatrix(row, column).determinant()

    fun cofactor(row: Int, column: Int): Double = minor(row, column) * cofactorSign(row, column)
    private fun cofactorSign(row: Int, column: Int) = if ((row + column) % 2 == 0) 1 else -1

    fun invertible() = determinant() != 0.0

    fun inverse(): Matrix {
        if (!invertible()) throw RuntimeException("Attempted to invert non-invertible matrix: $this")
        val determinant = determinant()
        val inverseValues: List<Double> = ((0 until width) * (0 until height)).map { (c, r) -> cofactor(r, c) / determinant }
        return Matrix(height, width, inverseValues)
    }

    /**
     * Returns the values from the specified column.
     */
    private fun getColumn(columnIndex: Int) = (0 until height).map { rowIndex -> get(rowIndex, columnIndex) }

    /**
     * This class represents an nxm matrix as a list of size nxm. This function calculates
     * the index into that list for the value at row & column.
     */
    private fun getIndexFor(row: Int, column: Int) = row * width + column

    /**
     * Helper for combining matrix transformations.
     *
     * A.then(B) will return B * A, which is the combined transformation to apply A and then B.
     */
    fun then(nextTransform: Matrix) = nextTransform * this

    /**
     * Helper class used by Matrix.ofSize().
     */
    class MatrixBuilder(private val rows: Int, private val columns: Int) {
        fun of(vararg values: Number) = Matrix(rows, columns, values.map(Number::toDouble))
    }
}
