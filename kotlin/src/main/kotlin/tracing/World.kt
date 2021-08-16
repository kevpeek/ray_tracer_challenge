package tracing

import display.Color
import geometry.Point
import geometry.WORLD_ORIGIN
import geometry.scaling

val BLACK_LIGHT = PointLight(WORLD_ORIGIN, Color.BLACK)

val DEFAULT_LIGHT = PointLight(Point(-10, -10, -10), Color.WHITE)
val DEFAULT_SPHERES = defaultSpheres()

/**
 * Logic to instantiate our default spheres.
 *
 * TODO - move all these values to defaults elsewhere.
 */
private fun defaultSpheres(): List<Sphere> {
    val outerSphereMaterial = Material(
        color = Color(0.8, 1.0, 0.6), diffuse = 0.7, specular = 0.2
    )
    val outerSphere = Sphere(material = outerSphereMaterial)
    val innerSphere = Sphere(scaling(0.5, 0.5, 0.5))
    return listOf(outerSphere, innerSphere)
}

class World(val objects: List<Sphere> = emptyList(), val lightSource: PointLight = BLACK_LIGHT) {
    companion object {
        fun default() = World(DEFAULT_SPHERES, DEFAULT_LIGHT)
    }

    /**
     * Returns all intersections of the supplied Ray with every object in the world.
     *
     * Resulting Intersections are sorted by time, so intersections from any given object may not adjacent in the list.
     */
    fun intersects(ray: Ray): List<Intersection> {
        return objects.flatMap { it.intersects(ray) }.sortedBy { it.time }
    }

    /**
     * Calculate the color produced by firing ray at this World.
     */
    fun colorAt(ray: Ray): Color {
        return hit(this.intersects(ray))?.preComputations(ray)
            ?.let(this::shadeHit)
            ?: Color.BLACK
    }

    /**
     * Determine the Color given a PreComputedIntersection.
     */
    fun shadeHit(preComputations: PreComputedIntersection): Color {
        return lighting(
            preComputations.thing.material,
            lightSource,
            preComputations.point,
            preComputations.eyeVector,
            preComputations.normalVector
        )
    }
}
