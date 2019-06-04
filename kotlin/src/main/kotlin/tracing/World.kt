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
        color = Color(0.8, 1.0, 0.6), diffuse = 0.7, specular = 0.2)
    val outerSphere = Sphere(material = outerSphereMaterial)
    val innerSphere = Sphere(scaling(0.5, 0.5, 0.5))
    return listOf(outerSphere, innerSphere)
}

class World(val objects: List<Sphere> = emptyList(), val lightSource: PointLight = BLACK_LIGHT) {

    companion object {
        fun default() = World(DEFAULT_SPHERES, DEFAULT_LIGHT)
    }
}