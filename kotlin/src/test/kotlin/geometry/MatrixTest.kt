package geometry

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.fail

class MatrixTest {

    @Test
    fun `Constructing and inspecting a 4x4 matrix`() {
        val matrix = Matrix(4, 4,
            1, 2, 3, 4,
            5.5, 6.5, 7.5, 8.5,
            9, 10, 11, 12,
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
        val matrix = Matrix(2, 2,
            -3, 5,
            1, -2
            )

        assertEquals(-3.0, matrix[0,0])
        assertEquals(5.0, matrix[0,1])
        assertEquals(1.0, matrix[1,0])
        assertEquals(-2.0, matrix[1,1])
    }

    @Test
    fun `A 3x3 matrix ought to be representable`() {
        val matrix = Matrix(3, 3,
            -3, 5, 0,
            1, -2, -7,
            0, 1, 1
        )

        assertEquals(-3.0, matrix[0,0])
        assertEquals(-2.0, matrix[1,1])
        assertEquals(1.0, matrix[2,2])
    }

    @Test
    fun `Matrix equality with identical matrices`() {
        val matrixA = Matrix(4, 4,
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        val matrixB = Matrix(4, 4,
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        assertEquals(matrixA, matrixB)
    }

    @Test
    fun `Matrix equality with different matrices`() {
        val matrixA = Matrix(4, 4,
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        val matrixB = Matrix(4, 4,
            2, 3, 4, 5,
            6, 7, 8, 9,
            8, 7, 6, 5,
            4, 3, 2, 1
        )

        assertNotEquals(matrixA, matrixB)
    }

    @Test
    fun `Multiplying two matrices`() {
        val matrixA = Matrix(4, 4,
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        val matrixB = Matrix(4, 4,
            -2, 1, 2, 3,
            3, 2, 1, -1,
            4, 3, 6, 5,
            1, 2, 7, 8
        )

        val expectedResult = Matrix(4, 4,
            20, 22, 50, 48,
            44, 54, 114, 108,
            40, 58, 110, 102,
            16, 26, 46, 42
        )

        assertEquals(expectedResult, matrixA * matrixB)
    }

    @Test
    fun `A matrix multiplied by a tuple`() {
        val matrix = Matrix(4, 4,
            1, 2, 3, 4,
            2, 4, 4, 2,
            8, 6, 4, 1,
            0, 0, 0, 1
        )

        val point = Point(1, 2, 3)

        assertEquals(Point(18, 24, 33), matrix * point)
    }

    @Test
    fun `Multiplying a matrix by the identity matrix`() {
        val matrix = Matrix(4, 4,
            0, 1, 2, 4,
            1, 2, 4, 8,
            2, 4, 8, 16,
            4, 8, 16, 32
            )

        assertEquals(matrix, matrix * Matrix.identity(4))
    }
}