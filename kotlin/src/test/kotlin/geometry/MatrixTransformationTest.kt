package geometry

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.fail

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

        assertEquals(Point(0, Math.sqrt(2.0) / 2, Math.sqrt(2.0) / 2), halfQuarter * point)
        assertEquals(Point(0, 0, 1), fullQuarter * point)
    }

    @Test
    fun `The inverse of an x-rotation rotates in the opposite direction`() {
        val point = Point(0, 1, 0)

        val halfQuarter = rotationX(Math.PI / 4)
        val inverse = halfQuarter.inverse()

        assertEquals(Point(0, Math.sqrt(2.0) / 2, - Math.sqrt(2.0) / 2), inverse * point)
    }

    @Test
    fun `Rotating a point around the y axis`() {
        val point = Point(0, 0, 1)

        val halfQuarter = rotationY(Math.PI / 4)
        val fullQuarter = rotationY(Math.PI / 2)

        assertEquals(Point(Math.sqrt(2.0) / 2, 0, Math.sqrt(2.0) / 2), halfQuarter * point)
        assertEquals(Point(1, 0, 0), fullQuarter * point)
    }

    @Test
    fun `Rotating a point around the z axis`() {
        val point = Point(0, 1, 0)

        val halfQuarter = rotationZ(Math.PI / 4)
        val fullQuarter = rotationZ(Math.PI / 2)

        assertEquals(Point(-Math.sqrt(2.0) / 2, Math.sqrt(2.0) / 2, 0), halfQuarter * point)
        assertEquals(Point(-1, 0, 0), fullQuarter * point)
    }
}