package tracing

import geometry.Point
import geometry.Vector
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class IntersectionTest {

    @Test
    fun `A ray intersects a sphere at two points`() {
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(4.0, intersections[0].t)
        assertEquals(6.0, intersections[1].t)
    }

    @Test
    fun `A ray intersects a sphere at a tangent`() {
        val ray = Ray(Point(0, 1, -5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(5.0, intersections[0].t)
        assertEquals(5.0, intersections[1].t)
    }

    @Test
    fun `A ray misses a sphere`() {
        val ray = Ray(Point(0, 2, -5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertTrue(intersections.isEmpty())
    }

    @Test
    fun `A ray originates inside a sphere`() {
        val ray = Ray(Point(0, 0, 0), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(-1.0, intersections[0].t)
        assertEquals(1.0, intersections[1].t)
    }

    @Test
    fun `A sphere is behind a ray`() {
        val ray = Ray(Point(0, 0, 5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(-6.0, intersections[0].t)
        assertEquals(-4.0, intersections[1].t)
    }

    @Test
    fun `An intersection encapsulates t and object`() {
        val sphere = Sphere()

        val intersection = Intersection(3.5, sphere)

        assertEquals(3.5, intersection.t)
        assertEquals(sphere, intersection.thing)
    }

    @Test
    fun `Aggregating intersections`() {
        val sphere = Sphere()

        val i1 = Intersection(1.0, sphere)
        val i2 = Intersection(2.0, sphere)

        val intersections = intersections(i1, i2)

        assertEquals(2, intersections.size)
        assertEquals(i1, intersections[0])
        assertEquals(i2, intersections[1])
    }

    @Test
    fun `Intersect sets the object on the intersection`() {
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(sphere, intersections[0].thing)
        assertEquals(sphere, intersections[1].thing)
    }
}