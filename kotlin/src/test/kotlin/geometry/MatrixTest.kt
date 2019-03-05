package geometry

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class MatrixTest {

    @Test
    fun `Constructing and inspecting a 4x4 matrix`() {
        val matrix = Matrix(
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5
        )

        assertEquals(1.0, matrix[0,0])
        assertEquals(4.0, matrix[0,3])
        assertEquals(5.5, matrix[1,0])
        assertEquals(7.5, matrix[1,2])
        assertEquals(11.0, matrix[2,2])
        assertEquals(13.5, matrix[3,0])
        assertEquals(15.5, matrix[3,2])
    }

    @Test
    fun `A 2x2 matrix ought to be representable`() {
        val matrix = Matrix(
            -3.0, 5.0,
            1.0, -2.0
            )

        assertEquals(-3.0, matrix[0,0])
        assertEquals(5.0, matrix[0,1])
        assertEquals(1.0, matrix[1,0])
        assertEquals(-2.0, matrix[1,1])
    }

    @Test
    fun `A 3x3 matrix ought to be representable`() {
        val matrix = Matrix(
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0
        )

        assertEquals(-3.0, matrix[0,0])
        assertEquals(-2.0, matrix[1,1])
        assertEquals(1.0, matrix[2,2])
    }

    @Test
    fun `Matrix equality with identical matrices`() {
        val matrixA = Matrix(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        )

        val matrixB = Matrix(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        )

        assertEquals(matrixA, matrixB)
    }

    @Test
    fun `Matrix equality with different matrices`() {
        val matrixA = Matrix(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        )

        val matrixB = Matrix(
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0
        )

        assertNotEquals(matrixA, matrixB)
    }
}