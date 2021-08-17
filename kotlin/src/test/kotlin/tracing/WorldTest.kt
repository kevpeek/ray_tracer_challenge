package tracing

import display.Color
import geometry.Point
import geometry.Vector
import geometry.scaling
import geometry.translation
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import kotlin.test.assertFalse

class WorldTest {

    @Test
    fun `Creating a world`() {
        val world = World()
        assertTrue(world.objects.isEmpty())
        assertEquals(BLACK_LIGHT, world.lightSource)
    }

    @Test
    fun `The default world`() {
        val defaultWorld = World.default()

        assertEquals(DEFAULT_LIGHT, defaultWorld.lightSource)
        assertEquals(DEFAULT_SPHERES, defaultWorld.objects)
    }

    @Test
    fun `Intersect a world with a ray`() {
        val world = World.default()
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))

        val intersections = world.intersects(ray)

        assertEquals(4, intersections.size)
        assertEquals(4.0, intersections[0].time)
        assertEquals(4.5, intersections[1].time)
        assertEquals(5.5, intersections[2].time)
        assertEquals(6.0, intersections[3].time)
    }

    @Test
    fun `Shading an intersection`() {
        val world = World.default()
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))
        val shape = world.objects.first()
        val intersect = shape.intersects(ray)[0]

        val comps = intersect.preComputations(ray)

        val color = world.shadeHit(comps)
        assertEquals(Color(0.38066, 0.47583, 0.2855), color)
    }

    @Test
    fun `Shading an intersection from the inside`() {
        val lightSource = PointLight(Point(0, 0.25, 0), Color(1, 1, 1))
        val world = World(DEFAULT_SPHERES, lightSource)
        val ray = Ray(Point(0, 0, 0), Vector(0, 0, 1))
        val shape = world.objects[1]
        val intersect = shape.intersects(ray)[1]

        val comps = intersect.preComputations(ray)

        val color = world.shadeHit(comps)
        assertEquals(Color(0.1, 0.1, 0.1), color)
    }

    @Test
    fun `The color when a ray misses`() {
        val world = World.default()
        val ray = Ray(Point(0, 0, -5), Vector(0, 1, 0))

        val color = world.colorAt(ray)
        assertEquals(Color.BLACK, color)
    }

    @Test
    fun `The color when a ray hits`() {
        val world = World.default()
        val ray = Ray(Point(0, 0, -5), Vector(0, 0, 1))

        val color = world.colorAt(ray)
        assertEquals(Color(0.38066, 0.47583, 0.28550), color)
    }

    @Test
    fun `The color with an intersection behind the ray`() {
        val outerSphereMaterial = Material(
            color = Color(0.8, 1.0, 0.6), ambient = 1.0, diffuse = 0.7, specular = 0.2
        )
        val outerSphere = Sphere(material = outerSphereMaterial)
        val innerSphere = Sphere(material = Material(ambient = 1.0)).withTransform { scaling(0.5, 0.5, 0.5) }

        val world = World(listOf(outerSphere, innerSphere), DEFAULT_LIGHT)

        val ray = Ray(Point(0, 0, 0.75), Vector(0, 0, -1))

        val color = world.colorAt(ray)
        assertEquals(innerSphere.material().color, color)
    }

    @Test
    fun no_shadow_when_nothing_collinear_with_point_and_light() {
        var world = World.default()
        var point = Point(0, 10, 0)
        assertFalse(world.isShadowed(point))
    }

    @Test
    fun shadow_when_something_between_point_and_light() {
        var world = World.default()
        var point = Point(10, -10, 10)
        assertTrue(world.isShadowed(point))
    }

    @Test
    fun no_shadow_when_object_behind_light() {
        var world = World.default()
        var point = Point(-20, 20, -20)
        assertFalse(world.isShadowed(point))
    }

    @Test
    fun no_shadow_when_object_behind_point() {
        var world = World.default()
        var point = Point(-2, 2, -2)
        assertFalse(world.isShadowed(point))
    }

    @Test
    fun shade_hit_given_intersection_in_shadow() {
        val shape1 = Sphere()
        val shape2 = Sphere().withTransform { translation(0, 0, 10) }
        val world = World(objects = listOf(shape1, shape2), PointLight(Point(0, 0, -10), Color.WHITE))
        val ray = Ray(Point(0, 0, 5), Vector(0, 0, 1))
        val intersection = Intersection(4.0, shape2)
        val preComputation = intersection.preComputations(ray)
        val color = world.shadeHit(preComputation)
        assertEquals(Color(0.1, 0.1, 0.1), color)
    }
}
