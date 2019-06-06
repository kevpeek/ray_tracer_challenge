package tracing

import geometry.Point
import geometry.Vector

/**
 * Precompute details about the intersection.
 */
data class PreComputedIntersection(
    val time: Double,
    val thing: Sphere,
    val inside: Boolean,
    val point: Point,
    val eyeVector: Vector,
    val normalVector: Vector
)

data class Intersection(val time: Double, val thing: Sphere) {
    /**
    * Calculate the PreComputed details.
    */
    fun preComputations(ray: Ray): PreComputedIntersection {
        val point = ray.position(time)
        val eyeVector = -ray.direction
        val normalVector = thing.normalAt(point).normalize()

        val inside = normalVector.dot(eyeVector) < 0
        val actualNormal = if (inside) -normalVector else normalVector

        return PreComputedIntersection(time, thing, inside, point, eyeVector, actualNormal)
    }
}

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
 * Returns all intersections of the supplied Ray with every object in the world.
 *
 * Resulting Intersections are sorted by time, so intersections from any given object may not adjacent in the list.
 */
fun intersectWorld(world: World, ray: Ray): List<Intersection> {
    return world.objects.flatMap { intersects(it, ray) }.sortedBy { it.time }
}

/**
 * Finds the Intersection with the lowest, non-negative time value.
 */
fun hit(intersections: List<Intersection>) = intersections.filter { it.time >= 0 }.minBy { it.time }
