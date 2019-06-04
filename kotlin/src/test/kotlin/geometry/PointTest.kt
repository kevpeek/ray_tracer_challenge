package geometry

import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class PointTest {

    @Test
    fun `Point sets x value`() {
        val point = Point(4.3, -4.2, 3.1)
        Assertions.assertEquals(4.3, point.x)
    }

    @Test
    fun `Point sets y value`() {
        val point = Point(4.3, -4.2, 3.1)
        Assertions.assertEquals(-4.2, point.y)
    }

    @Test
    fun `Point sets z value`() {
        val point = Point(4.3, -4.2, 3.1)
        Assertions.assertEquals(3.1, point.z)
    }

    @Test
    fun `equals() returns true for equivalent Points`() {
        val point1 = Point(1.0, 2.0, 3.0)
        val point2 = Point(1.0, 2.0, 3.0)
        Assertions.assertTrue(point1 == point2)
    }

    @Test
    fun `equals() returns true when values are within tolerance`() {
        val point1 = Point(1.000001, 2.0, 3.0)
        val point2 = Point(1.0, 2.0, 3.0)
        Assertions.assertTrue(point1 == point2)
    }

    @Test
    fun `equals() returns false for different Points`() {
        val point1 = Point(3.0, 1.0, 2.0)
        val point2 = Point(1.0, 2.0, 3.0)
        Assertions.assertFalse(point1 == point2)
    }

    @Test
    fun `adding point and vector yields correct point`() {
        val point = Point(3.0, -2.0, 5.0)
        val vector = Vector(-2.0, 3.0, 1.0)

        val newPoint = point + vector
        Assertions.assertEquals(Point(1.0, 1.0, 6.0), newPoint)
    }

    @Test
    fun`Subtracting two points`() {
        val point1 = Point(3.0, 2.0, 1.0)
        val point2 = Point(5.0, 6.0, 7.0)

        val newVector = point1 - point2
        Assertions.assertEquals(Vector(-2.0, -4.0, -6.0), newVector)
    }

    @Test
    fun`Subtracting a vector from a point`() {
        val point = Point(3.0, 2.0, 1.0)
        val vector = Vector(5.0, 6.0, 7.0)

        val newPoint = point - vector
        Assertions.assertEquals(Point(-2.0, -4.0, -6.0), newPoint)
    }
}