package tracing

import geometry.Point
import geometry.Vector
import geometry.scaling
import geometry.translation
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class IntersectionTest {

    @Test
    fun `A ray intersects a sphere at two points`() {
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(4.0, intersections[0].time)
        assertEquals(6.0, intersections[1].time)
    }

    @Test
    fun `A ray intersects a sphere at a tangent`() {
        val ray = Ray(Point(0, 1, -5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(5.0, intersections[0].time)
        assertEquals(5.0, intersections[1].time)
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
        assertEquals(-1.0, intersections[0].time)
        assertEquals(1.0, intersections[1].time)
    }

    @Test
    fun `A sphere is behind a ray`() {
        val ray = Ray(Point(0, 0, 5), Vector(0, 0, 1))
        val sphere = Sphere()

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(-6.0, intersections[0].time)
        assertEquals(-4.0, intersections[1].time)
    }

    @Test
    fun `An intersection encapsulates t and object`() {
        val sphere = Sphere()

        val intersection = Intersection(3.5, sphere)

        assertEquals(3.5, intersection.time)
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

    @Test
    fun `The hit, when all intersections have positive t`() {
        val shape = Sphere()
        val i1 = Intersection(1.0, shape)
        val i2 = Intersection(2.0, shape)
        val intersections = intersections(i1, i2)

        val theHit = hit(intersections)
        assertEquals(i1, theHit)
    }

    @Test
    fun `The hit, when some intersections have negative t`() {
        val shape = Sphere()

        val i1 = Intersection(-1.0, shape)
        val i2 = Intersection(1.0, shape)
        val intersections = intersections(i1, i2)

        val theHit = hit(intersections)
        assertEquals(i2, theHit)
    }

    @Test
    fun `The hit, when all intersections have negative t`() {
        val shape = Sphere()

        val i1 = Intersection(-2.0, shape)
        val i2 = Intersection(-1.0, shape)
        val intersections = intersections(i1, i2)

        val theHit = hit(intersections)
        assertNull(theHit)
    }

    @Test
    fun `The hit is always the lowest nonnegative intersection`() {
        val shape = Sphere()

        val i1 = Intersection(5.0, shape)
        val i2 = Intersection(7.0, shape)
        val i3 = Intersection(-3.0, shape)
        val i4 = Intersection(2.0, shape)
        val intersections = intersections(i1, i2, i3, i4)

        val theHit = hit(intersections)
        assertEquals(i4, theHit)
    }

    @Test
    fun `Intersecting a scaled sphere with a ray`() {
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val sphere = Sphere(scaling(2, 2, 2))

        val intersections = intersects(sphere, ray)

        assertEquals(2, intersections.size)
        assertEquals(3.0, intersections[0].time)
        assertEquals(7.0, intersections[1].time)
    }

    @Test
    fun `Intersecting a translated sphere with a ray`() {
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val sphere = Sphere(translation(5, 0, 0))

        val intersections = intersects(sphere, ray)
        assertTrue(intersections.isEmpty())
    }

    @Test
    fun `Precomputing the state of an intersection`() {
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val shape = Sphere()
        val intersection = intersects(shape, ray)[0]

        val comps = PreComputedIntersection.preComputations(intersection, ray)

        assertEquals(intersection.time, comps.time)
        assertEquals(intersection.thing, comps.thing)
        assertEquals(Point(0, 0, -1), comps.point)
        assertEquals(Vector(0, 0, -1), comps.eyeVector)
        assertEquals(Vector(0, 0, -1), comps.normalVector)
    }

    @Test
    fun `The hit, when an intersection occurs on the outside`() {
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val shape = Sphere()

        val intersect = intersects(shape, ray)[0]

        val comps = PreComputedIntersection.preComputations(intersect, ray)
        assertFalse(comps.inside)
    }

    @Test
    fun `The hit, when an intersection occurs on the inside`() {
        val ray = Ray(Point(0, 0, 0), Vector(0, 0, 1))
        val shape = Sphere()

        val intersect = intersects(shape, ray)[1]

        val comps = PreComputedIntersection.preComputations(intersect, ray)
        assertTrue(comps.inside)
        assertEquals(Point(0, 0, 1), comps.point)
        assertEquals(Vector(0, 0, -1), comps.eyeVector)
        assertEquals(Vector(0, 0, -1), comps.normalVector)
    }
}