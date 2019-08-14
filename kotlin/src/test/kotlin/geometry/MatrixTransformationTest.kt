package geometry

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import kotlin.math.sqrt

class MatrixTransformationTest {

    @Test
    fun `Multiplying by a translation matrix`() {
        val translation = translation(5, -3, 2)
        val point = Point(-3, 4, 5)

        assertEquals(Point(2, 1, 7), translation * point)
    }

    @Test
    fun `Multiplying by the inverse of a translation matrix`() {
        val translation = translation(5, -3, 2)
        val inverseTranslation = translation.inverse()

        val point = Point(-3, 4, 5)

        assertEquals(Point(-8, 7, 3), inverseTranslation * point)
    }

    @Test
    fun `Translation does not affect vectors`() {
        val translation = translation(5, -3, 2)
        val vector = Vector(-3, 4, 5)

        assertEquals(vector, translation * vector)
    }

    @Test
    fun `A scaling matrix applied to a point`() {
        val scaling = scaling(2, 3, 4)
        val point = Point(-4, 6, 8)

        assertEquals(Point(-8, 18, 32), scaling * point)
    }

    @Test
    fun `A scaling matrix applied to a vector`() {
        val scaling = scaling(2, 3, 4)
        val vector = Vector(-4, 6, 8)

        assertEquals(Vector(-8, 18, 32), scaling * vector)
    }

    @Test
    fun `Multiplying by the inverse of a scaling matrix`() {
        val scaling = scaling(2, 3, 4)
        val inverseScaling = scaling.inverse()
        val vector = Vector(-4, 6, 8)

        assertEquals(Vector(-2, 2, 2), inverseScaling * vector)
    }

    @Test
    fun `Reflection is scaling by a negative value`() {
        val xReflection = scaling(-1, 1, 1)
        val point = Point(2, 3, 4)

        assertEquals(Point(-2, 3, 4), xReflection * point)
    }

    @Test
    fun `Rotating a point around the x axis`() {
        val point = Point(0, 1, 0)

        val halfQuarter = rotationX(Math.PI / 4)
        val fullQuarter = rotationX(Math.PI / 2)

        assertEquals(Point(0, sqrt(2.0) / 2, sqrt(2.0) / 2), halfQuarter * point)
        assertEquals(Point(0, 0, 1), fullQuarter * point)
    }

    @Test
    fun `The inverse of an x-rotation rotates in the opposite direction`() {
        val point = Point(0, 1, 0)

        val halfQuarter = rotationX(Math.PI / 4)
        val inverse = halfQuarter.inverse()

        assertEquals(Point(0, sqrt(2.0) / 2, - sqrt(2.0) / 2), inverse * point)
    }

    @Test
    fun `Rotating a point around the y axis`() {
        val point = Point(0, 0, 1)

        val halfQuarter = rotationY(Math.PI / 4)
        val fullQuarter = rotationY(Math.PI / 2)

        assertEquals(Point(sqrt(2.0) / 2, 0, sqrt(2.0) / 2), halfQuarter * point)
        assertEquals(Point(1, 0, 0), fullQuarter * point)
    }

    @Test
    fun `Rotating a point around the z axis`() {
        val point = Point(0, 1, 0)

        val halfQuarter = rotationZ(Math.PI / 4)
        val fullQuarter = rotationZ(Math.PI / 2)

        assertEquals(Point(-sqrt(2.0) / 2, sqrt(2.0) / 2, 0), halfQuarter * point)
        assertEquals(Point(-1, 0, 0), fullQuarter * point)
    }

    @Test
    fun `A shearing transformation moves x in proportion to y`() {
        val point = Point(2, 3, 4)

        val transformation = shearing(1, 0, 0, 0, 0, 0)

        assertEquals(Point(5, 3, 4), transformation * point)
    }

    @Test
    fun `A shearing transformation moves x in proportion to z`() {
        val point = Point(2, 3, 4)

        val transformation = shearing(0, 1, 0, 0, 0, 0)

        assertEquals(Point(6, 3, 4), transformation * point)
    }

    @Test
    fun `A shearing transformation moves y in proportion to x`() {
        val point = Point(2, 3, 4)

        val transformation = shearing(0, 0, 1, 0, 0, 0)

        assertEquals(Point(2, 5, 4), transformation * point)
    }

    @Test
    fun `A shearing transformation moves y in proportion to z`() {
        val point = Point(2, 3, 4)

        val transformation = shearing(0, 0, 0, 1, 0, 0)

        assertEquals(Point(2, 7, 4), transformation * point)
    }

    @Test
    fun `A shearing transformation moves z in proportion to x`() {
        val point = Point(2, 3, 4)

        val transformation = shearing(0, 0, 0, 0, 1, 0)

        assertEquals(Point(2, 3, 6), transformation * point)
    }

    @Test
    fun `A shearing transformation moves z in proportion to y`() {
        val point = Point(2, 3, 4)

        val transformation = shearing(0, 0, 0, 0, 0, 1)

        assertEquals(Point(2, 3, 7), transformation * point)
    }

    @Test
    fun `Individual transformations are applied in sequence`() {
        val point = Point(1, 0, 1)

        val transformA = rotationX(Math.PI / 2)
        val transformB = scaling(5, 5, 5)
        val transformC = translation(10, 5, 7)

        val point2 = transformA * point
        assertEquals(Point(1, -1, 0), point2)

        val point3 = transformB * point2
        assertEquals(Point(5, -5, 0), point3)

        val point4 = transformC * point3
        assertEquals(Point(15, 0, 7), point4)
    }

    @Test
    fun `Chained transformations must be applied in reverse order`() {
        val point = Point(1, 0, 1)

        val transformA = rotationX(Math.PI / 2)
        val transformB = scaling(5, 5, 5)
        val transformC = translation(10, 5, 7)

        val combinedTransform = transformC * transformB * transformA

        assertEquals(Point(15, 0, 7), combinedTransform * point)
    }

    @Test
    fun `Fluently chained transformations must be applied in reverse order`() {
        val point = Point(1, 0, 1)

        val transformA = rotationX(Math.PI / 2)
        val transformB = scaling(5, 5, 5)
        val transformC = translation(10, 5, 7)

        val combinedTransform = transformA.then(transformB).then(transformC)

        assertEquals(Point(15, 0, 7), combinedTransform * point)
    }

    @Test
    fun `The transformation matrix for the default orientation`() {
        val from = Point(0, 0, 0)
        val to = Point(0, 0, -1)
        val up = Vector(0, 1, 0)

        val result = viewTransform(from, to, up)
        assertEquals(Matrix.identity(4), result)
    }

    @Test
    fun `A view transformation matrix looking in positive z direction`() {
        val from = Point(0, 0, 0)
        val to = Point(0, 0, 1)
        val up = Vector(0, 1, 0)

        val result = viewTransform(from, to, up)
        assertEquals(scaling(-1, 1, -1), result)
    }

    @Test
    fun `The view transformation moves the world`() {
        val from = Point(0, 0, 8)
        val to = Point(0, 0, 0)
        val up = Vector(0, 1, 0)

        val result = viewTransform(from, to, up)
        assertEquals(translation(0, 0, -8), result)
    }

    @Test
    fun `An arbitrary view transformation`() {
        val from = Point(1, 3, 2)
        val to = Point(4, -2, 8)
        val up = Vector(1, 1, 0)

        val result = viewTransform(from, to, up)

        val expectedResult = Matrix.ofSize(4, 4).of(
            -0.50709, 0.50709, 0.67612, -2.36643,
            0.76772, 0.60609, 0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714, 0,
            0, 0, 0, 1
        )

        assertEquals(expectedResult, result)
    }
}