package geometry

import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import kotlin.math.sqrt

class VectorTest {

    @Test
    fun `Vector sets x value`() {
        val tuple = Vector(4.3, -4.2, 3.1)
        Assertions.assertEquals(4.3, tuple.x)
    }

    @Test
    fun `Vector sets y value`() {
        val tuple = Vector(4.3, -4.2, 3.1)
        Assertions.assertEquals(-4.2, tuple.y)
    }

    @Test
    fun `Vector sets z value`() {
        val tuple = Vector(4.3, -4.2, 3.1)
        Assertions.assertEquals(3.1, tuple.z)
    }

    @Test
    fun `equals() returns true for equivalent Vector`() {
        val tuple1 = Vector(1.0, 2.0, 3.0)
        val tuple2 = Vector(1.0, 2.0, 3.0)
        Assertions.assertTrue(tuple1 == tuple2)
    }

    @Test
    fun `equals() returns true when values are within tolerance`() {
        val tuple1 = Vector(1.000001, 2.0, 3.0)
        val tuple2 = Vector(1.0, 2.0, 3.0)
        Assertions.assertTrue(tuple1 == tuple2)
    }

    @Test
    fun `equals() returns false for different Vectors`() {
        val tuple1 = Vector(3.0, 1.0, 2.0)
        val tuple2 = Vector(1.0, 2.0, 3.0)
        Assertions.assertFalse(tuple1 == tuple2)
    }

    @Test
    fun `adding vector and vector yields correct vector`() {
        val vector1 = Vector(3.0, -2.0, 5.0)
        val vector2 = Vector(-2.0, 3.0, 1.0)

        val newVector = vector1 + vector2
        Assertions.assertEquals(Vector(1.0, 1.0, 6.0), newVector)
    }

    @Test
    fun `Subtracting two vectors`() {
        val vector1 = Vector(3.0, 2.0, 1.0)
        val vector2 = Vector(5.0, 6.0, 7.0)

        val newVector = vector1 - vector2
        Assertions.assertEquals(Vector(-2.0, -4.0, -6.0), newVector)
    }

    @Test
    fun `Subtracting a vector from the zero vector`() {
        val vector1 = Vector(0.0, 0.0, 0.0)
        val vector2 = Vector(1.0, -2.0, 3.0)

        val newVector = vector1 - vector2
        Assertions.assertEquals(Vector(-1.0, 2.0, -3.0), newVector)
    }

    @Test
    fun `Negating a Vector`() {
        val vector = Vector(1.0, -2.0, 3.0)
        val negativeVector = -vector
        Assertions.assertEquals(Vector(-1.0, 2.0, -3.0), negativeVector)
    }

    @Test
    fun `Multiplying a Vector by a scalar`() {
        val tuple = Vector(1.0, -2.0, 3.0)

        val newTuple = tuple * 3.5
        Assertions.assertEquals(Vector(3.5, -7.0, 10.5), newTuple)
    }

    @Test
    fun `Multiplying a Vector by a fraction`() {
        val tuple = Vector(1.0, -2.0, 3.0)

        val newTuple = tuple * 0.5
        Assertions.assertEquals(Vector(0.5, -1.0, 1.5), newTuple)
    }

    @Test
    fun `Dividing a Vector by a scalar`() {
        val tuple = Vector(1.0, -2.0, 3.0)
        val newTuple = tuple / 2.0
        Assertions.assertEquals(Vector(0.5, -1.0, 1.5), newTuple)
    }

    @Test
    fun `Computing the magnitude of Vector(1, 0, 0)`() {
        val vector = Vector(1.0, 0.0, 0.0)
        Assertions.assertEquals(1.0, vector.magnitude())
    }

    @Test
    fun `Computing the magnitude of Vector(0, 1, 0)`() {
        val vector = Vector(0.0, 1.0, 0.0)
        Assertions.assertEquals(1.0, vector.magnitude())
    }

    @Test
    fun `Computing the magnitude of Vector(0, 0, 1)`() {
        val vector = Vector(0.0, 0.0, 1.0)
        Assertions.assertEquals(1.0, vector.magnitude())
    }

    @Test
    fun `Computing the magnitude of Vector(1, 2, 3)`() {
        val vector = Vector(1.0, 2.0, 3.0)
        Assertions.assertEquals(sqrt(14.0), vector.magnitude())
    }

    @Test
    fun `Computing the magnitude of Vector(-1, -2, -3)`() {
        val vector = Vector(-1.0, -2.0, -3.0)
        Assertions.assertEquals(sqrt(14.0), vector.magnitude())
    }

    @Test
    fun `Normalizing Vector(4, 0, 0) gives (1, 0, 0)`() {
        val vector = Vector(4.0, 0.0, 0.0)
        Assertions.assertEquals(Vector(1.0, 0.0, 0.0), vector.normalize())
    }

    @Test
    fun `Normalizing Vector(1, 2, 3)`() {
        val vector = Vector(1.0, 2.0, 3.0)
        val sqrt14 = sqrt(14.0)
        Assertions.assertEquals(Vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14), vector.normalize())
    }

    @Test
    fun `The magnitude of a normalized Vector should be 1`() {
        val vector = Vector(1.0, 2.0, 3.0)
        val normalizedVector = vector.normalize()
        Assertions.assertEquals(1.0, normalizedVector.magnitude())
    }

    @Test
    fun `The dot product of two Vectors`() {
        val vector1 = Vector(1.0, 2.0, 3.0)
        val vector2 = Vector(2.0, 3.0, 4.0)
        Assertions.assertEquals(20.0, vector1.dot(vector2))
    }

    @Test
    fun `The cross product of two Vector`() {
        val vector1 = Vector(1.0, 2.0, 3.0)
        val vector2 = Vector(2.0, 3.0, 4.0)

        val cross12 = vector1.cross(vector2)
        Assertions.assertEquals(Vector(-1.0, 2.0, -1.0), cross12)

        val cross21 = vector2.cross(vector1)
        Assertions.assertEquals(Vector(1.0, -2.0, 1.0), cross21)
    }

    @Test
    fun `Reflecting a vector approaching at 45Â°`() {
        val vector = Vector(1, -1, 0)
        val normal = Vector(0, 1, 0)

        val reflection = vector.reflect(normal)

        assertEquals(Vector(1, 1, 0), reflection)
    }

    @Test
    fun `Reflecting a vector off a slanted surface`() {
        val vector = Vector(0, -1, 0)
        val normal = Vector(sqrt(2.0) / 2, sqrt(2.0) / 2, 0)

        val reflection = vector.reflect(normal)

        assertEquals(Vector(1, 0, 0), reflection)
    }
}
