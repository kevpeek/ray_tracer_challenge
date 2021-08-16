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
    val transform: Matrix = Matrix.identity(4),
    val origin: Point = WORLD_ORIGIN,
    val material: Material = Material.DEFAULT
) {

    /**
     * Returns the list of Intersections between the ray and sphere.
     */
    fun intersects(ray: Ray): List<Intersection> {
        val transformedRay = ray.transform(transform.inverse())
        val sphereToRay = transformedRay.origin - origin
        val a = transformedRay.direction.dot(transformedRay.direction)
        val b = 2 * transformedRay.direction.dot(sphereToRay)
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
    fun normalAt(point: Point): Vector {
        val transformToObjectSpace = transform.inverse()
        val pointInObjectSpace = transformToObjectSpace * point
        val normalInObjectSpace = (pointInObjectSpace - origin)
        val transformToWorldSpace = transform.submatrix(3, 3).inverse().transpose()
        return (transformToWorldSpace * normalInObjectSpace).normalize()
    }

    fun withTransform(otherTransform: () -> Matrix) = Sphere(otherTransform(), origin, material)
    fun withOrigin(otherOrigin: Point) = Sphere(transform, otherOrigin, material)
    fun withMaterial(otherMaterial: Material) = Sphere(transform, origin, otherMaterial)
}
