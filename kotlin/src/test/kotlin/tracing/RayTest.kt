package tracing

import geometry.Point
import geometry.Vector
import geometry.scaling
import geometry.translation
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

    @Test
    fun `Translating a ray`() {
        val ray = Ray(Point(1, 2, 3), Vector(0, 1, 0))
        val matrix = translation(3, 4, 5)

        val ray2 = ray.transform(matrix)
        assertEquals(Point(4, 6, 8), ray2.origin)
        assertEquals(Vector(0, 1, 0), ray2.direction)
    }

    @Test
    fun `Scaling a ray`() {
        val ray = Ray(Point(1, 2, 3), Vector(0, 1, 0))
        val matrix = scaling(2, 3, 4)

        val ray2 = ray.transform(matrix)
        assertEquals(Point(2, 6, 12), ray2.origin)
        assertEquals(Vector(0, 3, 0), ray2.direction)
    }
}