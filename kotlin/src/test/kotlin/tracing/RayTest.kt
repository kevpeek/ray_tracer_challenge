package tracing

import geometry.Point
import geometry.Vector
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.fail

class RayTest {

    @Test
    fun `Creating and querying a ray`() {
        val origin = Point(1, 2, 3)
        val direction = Vector(4, 5, 6)

        val ray = Ray(origin, direction)

        assertEquals(origin, ray.origin)
        assertEquals(direction, ray.direction)
    }

    @Test
    fun `Computing a point from a distance`() {
        val ray = Ray(Point(2, 3, 4), Vector(1, 0, 0))

        assertEquals(Point(2, 3, 4), ray.position(0))
        assertEquals(Point(3, 3, 4), ray.position(1))
        assertEquals(Point(1, 3, 4), ray.position(-1))
        assertEquals(Point(4.5, 3, 4), ray.position(2.5))
    }
}