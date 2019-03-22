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
        val matrix = Matrix.ofSize(4, 4).of(
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
        val matrix = Matrix.ofSize(2, 2).of(
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
        val matrix = Matrix.ofSize(3, 3).of(
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
        val matrixA = Matrix.ofSize(4, 4).of(
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        val matrixB = Matrix.ofSize(4, 4).of(
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        assertEquals(matrixA, matrixB)
    }

    @Test
    fun `Matrix equality with different matrices`() {
        val matrixA = Matrix.ofSize(4, 4).of(
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        val matrixB = Matrix.ofSize(4, 4).of(
            2, 3, 4, 5,
            6, 7, 8, 9,
            8, 7, 6, 5,
            4, 3, 2, 1
        )

        assertNotEquals(matrixA, matrixB)
    }

    @Test
    fun `Multiplying two matrices`() {
        val matrixA = Matrix.ofSize(4, 4).of(
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 8, 7, 6,
            5, 4, 3, 2
        )

        val matrixB = Matrix.ofSize(4, 4).of(
            -2, 1, 2, 3,
            3, 2, 1, -1,
            4, 3, 6, 5,
            1, 2, 7, 8
        )

        val expectedResult = Matrix.ofSize(4, 4).of(
            20, 22, 50, 48,
            44, 54, 114, 108,
            40, 58, 110, 102,
            16, 26, 46, 42
        )

        assertEquals(expectedResult, matrixA * matrixB)
    }

    @Test
    fun `A matrix multiplied by a tuple`() {
        val matrix = Matrix.ofSize(4, 4).of(
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
        val matrix = Matrix.ofSize(4, 4).of(
            0, 1, 2, 4,
            1, 2, 4, 8,
            2, 4, 8, 16,
            4, 8, 16, 32
            )

        assertEquals(matrix, matrix * Matrix.identity(4))
    }

    @Test
    fun `Multiplying the identity matrix by a tuple`() {
        val point = Point(1, 2, 3)
        assertEquals(point, Matrix.identity(4) * point)
    }


    @Test
    fun `Transposing a matrix`() {
        val matrix = Matrix.ofSize(4, 4).of(
            0, 9, 3, 0,
            9, 8, 0, 8,
            1, 8, 5, 3,
            0, 0, 5, 8
        )

        val expectedTranspose = Matrix.ofSize(4, 4).of(
            0, 9, 1, 0,
            9, 8, 8, 0,
            3, 0, 5, 5,
            0, 8, 3, 8
        )

        assertEquals(expectedTranspose, matrix.transpose())
    }

    @Test
    fun `Transposing the identity matrix`() {
        val identity = Matrix.identity(4)
        assertEquals(identity, identity.transpose())
    }

    @Test
    fun `Calculating the determinant of a 2x2 matrix`() {
        val matrix = Matrix.ofSize(2, 2).of(
            1, 5,
            -3, 2
        )

        assertEquals(17.0, matrix.determinant())
    }

    @Test
    fun `A submatrix of a 3x3 matrix is a 2x2 matrix`() {
        val matrix = Matrix.ofSize(3, 3).of(
            1, 5, 0,
            -3, 2, 7,
            0, 6, -3
        )

        val expectedSub = Matrix.ofSize(2, 2).of(
            -3, 2,
            0, 6
        )

        assertEquals(expectedSub, matrix.submatrix(0, 2))
    }

    @Test
    fun `A submatrix of a 4x4 matrix is a 3x3 matrix`() {
        val matrix = Matrix.ofSize(4, 4).of(
            -6, 1, 1, 6,
            -8, 5, 8, 6,
            -1, 0, 8, 2,
            -7, 1, -1, 1
        )

        val expectedSub = Matrix.ofSize(3, 3).of(
            -6, 1, 6,
            -8, 8, 6,
            -7, -1, 1
        )

        assertEquals(expectedSub, matrix.submatrix(2, 1))
    }
}