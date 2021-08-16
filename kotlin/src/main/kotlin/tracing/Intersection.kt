package tracing

import geometry.Point
import geometry.Vector

/**
 * Precompute details about the intersection.
 */
data class PreComputedIntersection(
    val time: Double,
    val thing: Shape,
    val inside: Boolean,
    val point: Point,
    val eyeVector: Vector,
    val normalVector: Vector
)

data class Intersection(val time: Double, val thing: Shape) {
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
 * Finds the Intersection with the lowest, non-negative time value.
 */
fun List<Intersection>.hit() = this.filter { it.time >= 0 }.minByOrNull { it.time }
