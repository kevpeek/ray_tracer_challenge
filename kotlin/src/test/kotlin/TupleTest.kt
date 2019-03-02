import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.fail

class TupleTest {

    @Test
    fun `Tuple sets x value`() {
        val tuple = Tuple(4.3, -4.2, 3.1, 1.0)
        assertEquals(4.3, tuple.x)
    }

    @Test
    fun `Tuple sets y value`() {
        val tuple = Tuple(4.3, -4.2, 3.1, 1.0)
        assertEquals(-4.2, tuple.y)
    }

    @Test
    fun `Tuple sets z value`() {
        val tuple = Tuple(4.3, -4.2, 3.1, 1.0)
        assertEquals(3.1, tuple.z)
    }

    @Test
    fun `Tuple with w = 1 is a point`() {
        val tuple = Tuple(4.3, -4.2, 3.1, 1.0)
        assertTrue(tuple.isPoint())
        assertFalse(tuple.isVector())
    }

    @Test
    fun `Tuple with w = 0 is a vector`() {
        val tuple = Tuple(4.3, -4.2, 3.1, 0.0)
        assertFalse(tuple.isPoint())
        assertTrue(tuple.isVector())
    }

    @Test
    fun `equals() returns true for equivalent tuples`() {
        val tuple1 = Tuple(1.0, 2.0, 3.0, 1.0)
        val tuple2 = Tuple(1.0, 2.0, 3.0, 1.0)
        assertTrue(tuple1 == tuple2)
    }

    @Test
    fun `equals() returns true when values are within tolerance`() {
        val tuple1 = Tuple(1.000001, 2.0, 3.0, 1.0)
        val tuple2 = Tuple(1.0, 2.0, 3.0, 1.0)
        assertTrue(tuple1 == tuple2)
    }

    @Test
    fun `equals() returns false for different tuples`() {
        val tuple1 = Tuple(3.0, 1.0, 2.0, 0.0)
        val tuple2 = Tuple(1.0, 2.0, 3.0, 1.0)
        assertFalse(tuple1 == tuple2)
    }

    @Test
    fun `point() creates a tuple with w=1`() {
        val tuple = Tuple.point(4.0, -4.0, 3.0)
        assertTrue(tuple.isPoint())
    }

    @Test
    fun `vector() creates tuple with w=0`() {
        val tuple = Tuple.vector(4.0, -4.0, 3.0)
        assertTrue(tuple.isVector())
    }

    @Test
    fun `adding point and vector yields correct point`() {
        val point = Tuple.point(3.0, -2.0, 5.0)
        val vector = Tuple.vector(-2.0, 3.0, 1.0)

        val newPoint = point.add(vector)
        assertEquals(Tuple.point(1.0, 1.0, 6.0), newPoint)
    }

    @Test
    fun `adding vector and vector yields correct vector`() {
        val vector1 = Tuple.vector(3.0, -2.0, 5.0)
        val vector2 = Tuple.vector(-2.0, 3.0, 1.0)

        val newVector = vector1.add(vector2)
        assertEquals(Tuple.vector(1.0, 1.0, 6.0), newVector)
    }

    @Test
    fun`Subtracting two points`() {
        val point1 = Tuple.point(3.0, 2.0, 1.0)
        val point2 = Tuple.point(5.0, 6.0, 7.0)

        val newVector = point1.minus(point2)
        assertEquals(Tuple.vector(-2.0, -4.0, -6.0), newVector)
    }

    @Test
    fun`Subtracting a vector from a point`() {
        val point = Tuple.point(3.0, 2.0, 1.0)
        val vector = Tuple.vector(5.0, 6.0, 7.0)

        val newPoint = point.minus(vector)
        assertEquals(Tuple.point(-2.0, -4.0, -6.0), newPoint)
    }

    @Test
    fun `Subtracting two vectors`() {
        val vector1 = Tuple.vector(3.0, 2.0, 1.0)
        val vector2 = Tuple.vector(5.0, 6.0, 7.0)

        val newVector = vector1.minus(vector2)
        assertEquals(Tuple.vector(-2.0, -4.0, -6.0), newVector)
    }

    @Test
    fun `Subtracting a vector from the zero vector`() {
        val vector1 = Tuple.vector(0.0, 0.0, 0.0)
        val vector2 = Tuple.vector(1.0, -2.0, 3.0)

        val newVector = vector1.minus(vector2)
        assertEquals(Tuple.vector(-1.0, 2.0, -3.0), newVector)
    }
}