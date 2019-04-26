package tracing

import geometry.Point

data class Intersection(val time: Double, val thing: Sphere)

fun intersections(vararg intersections: Intersection) = intersections.toList()

/**
 * Returns the list of Intersections between the ray and sphere.
 */
fun intersects(sphere: Sphere, ray: Ray): List<Intersection> {
    val transformedRay = ray.transform(sphere.transform.inverse())
    val sphereToRay = transformedRay.origin - sphere.origin
    val a = transformedRay.direction.dot(transformedRay.direction)
    val b = 2 * transformedRay.direction.dot(sphereToRay)
    val c = sphereToRay.dot(sphereToRay) - 1

    val discriminant = b * b - 4 * a * c

    if (discriminant < 0.0) {
        return emptyList()
    }

    val t1 = (-b - Math.sqrt(discriminant)) / (2 * a)
    val t2 = (-b + Math.sqrt(discriminant)) / (2 * a)

    return listOf(Intersection(t1, sphere), Intersection(t2, sphere))
}

/**
 * Finds the Intersection with the lowest, non-negative time value.
 */
fun hit(intersections: List<Intersection>) = intersections.filter { it.time >= 0 }.minBy { it.time }