package geometry

import helper.approximately
import helper.times
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

    @Test
    fun `Calculating a minor of a 3x3 matrix`() {
        val matrix = Matrix.ofSize(3, 3).of(
            3, 5, 0,
            2, -1, -7,
            6, -1, 5
        )

        val submatrix = matrix.submatrix(1, 0)

        assertEquals(25.0, submatrix.determinant())

        assertEquals(25.0, matrix.minor(1, 0))
    }

    @Test
    fun `Calculating a cofactor of a 3x3 matrix`() {
        val matrix = Matrix.ofSize(3, 3).of(
            3, 5, 0,
            2, -1, -7,
            6, -1, 5
        )

        assertEquals(-12.0, matrix.minor(0, 0))
        assertEquals(-12.0, matrix.cofactor(0, 0))
        assertEquals(25.0, matrix.minor(1, 0))
        assertEquals(-25.0, matrix.cofactor(1, 0))
    }

    @Test
    fun `Calculating the determinant of a 3x3 matrix`() {
        val matrix = Matrix.ofSize(3, 3).of(
            1, 2, 6,
            -5, 8, -4,
            2, 6, 4
        )

        assertEquals(56.0, matrix.cofactor(0, 0))
        assertEquals(12.0, matrix.cofactor(0, 1))
        assertEquals(-46.0, matrix.cofactor(0, 2))
        assertEquals(-196.0, matrix.determinant())
    }

    @Test
    fun `Calculating the determinant of a 4x4 matrix`() {
        val matrix = Matrix.ofSize(4, 4).of(
            -2, -8, 3, 5,
            -3, 1, 7, 3,
            1, 2, -9, 6,
            -6, 7, 7, -9
        )

        assertEquals(690.0, matrix.cofactor(0, 0))
        assertEquals(447.0, matrix.cofactor(0, 1))
        assertEquals(210.0, matrix.cofactor(0, 2))
        assertEquals(51.0, matrix.cofactor(0, 3))
        assertEquals(-4071.0, matrix.determinant())
    }

    @Test
    fun `Testing an invertible matrix for invertibility`() {
        val matrix = Matrix.ofSize(4, 4).of(
            6, 4, 4, 4,
            5, 5, 7, 6,
            4, -9, 3, -7,
            9, 1, 7, -6
        )

        assertEquals(-2120.0, matrix.determinant())
        assertTrue(matrix.invertible())
    }

    @Test
    fun `Testing a noninvertible matrix for invertibility`() {
        val matrix = Matrix.ofSize(4, 4).of(
            -4, 2, -2, -3,
            9, 6, 2, 6,
            0, -5, 1, -5,
            0, 0, 0, 0
        )

        assertEquals(0.0, matrix.determinant())
        assertFalse(matrix.invertible())
    }

    @Test
    fun `Calculating the inverse of a matrix`() {
        val matrix = Matrix.ofSize(4, 4).of(
            -5, 2, 6, -8,
            1, -5, 1, 8,
            7, 7, -6, -7,
            1, -3, 7, 4
        )

        val inverse = matrix.inverse()

        assertEquals(532.0, matrix.determinant())
        assertEquals(-160.0, matrix.cofactor(2, 3))
        assertEquals(-160.0/532.0, inverse[3, 2])
        assertEquals(105.0, matrix.cofactor(3, 2))
        assertEquals(105.0/532.0, inverse[2, 3])

        val expectedInverse = Matrix.ofSize(4, 4).of(
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639
        )

        assertEquals(expectedInverse, inverse)
    }

    @Test
    fun `Calculating the inverse of another matrix`() {
        val matrix = Matrix.ofSize(4, 4).of(
            8, -5, 9, 2,
            7, 5, 6, 1,
            -6, 0, 9, 6,
            -3, 0, -9, -4
        )

        val expectedInverse = Matrix.ofSize(4, 4).of(
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308
        )

        assertEquals(expectedInverse, matrix.inverse())
    }

    @Test
    fun `Calculating the inverse of a third matrix`() {
        val matrix = Matrix.ofSize(4, 4).of(
            9, 3, 0, 9,
            -5, -2, -6, -3,
            -4, 9, 6, 4,
            -7, 6, 6, 2
        )

        val expectedInverse = Matrix.ofSize(4, 4).of(
            -0.04074, -0.07778,  0.14444, -0.22222,
            -0.07778, 0.03333,  0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926,  0.12963,
            0.17778,  0.06667, -0.26667,  0.33333
        )

        assertEquals(expectedInverse, matrix.inverse())
    }

    @Test
    fun `Multiplying a product by its inverse`() {
        val matrixA = Matrix.ofSize(4, 4).of(
            3, -9, 7, 3,
            3, -8, 2, -9,
            -4, 4, 4, 1,
            -6, 5, -1, 1
        )

        val matrixB = Matrix.ofSize(4, 4).of(
            8, 2, 2, 2,
            3, -1, 7, 0,
            7, 0, 5, 4,
            6, -2, 0, 5
        )

        val matrixC = matrixA * matrixB

        assertEquals(matrixA, matrixC * matrixB.inverse())
    }

    @Test
    fun `Inverting the Identity matrix`() {
        val identity4 = Matrix.identity(4)
        assertEquals(identity4, identity4.inverse())
    }

    @Test
    fun `multiplying matrix by its inverse`() {
        val matrix = Matrix.ofSize(4, 4).of(
            3, -9, 7, 3,
            3, -8, 2, -9,
            -4, 4, 4, 1,
            -6, 5, -1, 1
        )

        assertEquals(Matrix.identity(4), matrix * matrix.inverse())
    }

    @Test
    fun `transpose of inverse equals inverse of transpose`() {
        val matrix = Matrix.ofSize(4, 4).of(
            3, -9, 7, 3,
            3, -8, 2, -9,
            -4, 4, 4, 1,
            -6, 5, -1, 1
        )

        assertEquals(matrix.transpose().inverse(), matrix.inverse().transpose())
    }
}