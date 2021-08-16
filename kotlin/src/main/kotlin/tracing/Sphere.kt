package tracing

import geometry.Matrix
import geometry.Point
import geometry.Vector
import geometry.WORLD_ORIGIN
import kotlin.math.sqrt

/**
 * Representation of a Sphere.
 */
class Sphere(
    val origin: Point = WORLD_ORIGIN,
    val material: Material = Material.DEFAULT,
) : Shape {

    override fun material() = material

    /**
     * Returns the list of Intersections between the ray and sphere.
     */
    override fun intersects(ray: Ray): List<Intersection> {
        val sphereToRay = ray.origin - origin
        val a = ray.direction.dot(ray.direction)
        val b = 2 * ray.direction.dot(sphereToRay)
        val c = sphereToRay.dot(sphereToRay) - 1

        val discriminant = b * b - 4 * a * c

        if (discriminant < 0.0) {
            return emptyList()
        }

        val t1 = (-b - sqrt(discriminant)) / (2 * a)
        val t2 = (-b + sqrt(discriminant)) / (2 * a)

        return listOf(Intersection(t1, this), Intersection(t2, this))
    }

    /**
     * Return the Vector normal to this sphere at the supplied point.
     */
    override fun normalAt(point: Point): Vector {
        return point - origin
    }

    fun withOrigin(otherOrigin: Point) = Sphere(otherOrigin, material)
    fun withMaterial(otherMaterial: Material) = Sphere(origin, otherMaterial)
}
