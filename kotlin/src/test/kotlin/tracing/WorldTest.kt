package tracing

import geometry.Point
import geometry.Vector
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.fail

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

        val intersections = intersectWorld(world, ray)

        assertEquals(4, intersections.size)
        assertEquals(4.0, intersections[0].time)
        assertEquals(4.5, intersections[1].time)
        assertEquals(5.5, intersections[2].time)
        assertEquals(6.0, intersections[3].time)
    }

    @Test
    fun `Shading an intersection`() {
        fail("todo")
        /*
  Given w â† default_world()
    And r â† ray(point(0, 0, -5), vector(0, 0, 1))
    And shape â† the first object in w
    And i â† intersection(4, shape)
  When comps â† prepare_computations(i, r)
    And c â† shade_hit(w, comps)
  Then c = color(0.38066, 0.47583, 0.2855)
        */
    }
}