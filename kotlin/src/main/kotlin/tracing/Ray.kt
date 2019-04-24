package tracing

import geometry.Matrix
import geometry.Point
import geometry.Vector

/**
 * Representation of a Ray traveling away from origin.
 */
data class Ray(val origin: Point, val direction: Vector) {

    /**
     * Calculates the position of the Ray after the specified amount of time.
     */
    fun position(time: Number) = origin + (direction * time.toDouble())

    /**
     * Return the Ray obtained by applying the supplied transformation to this Ray.
     */
    fun transform(transformation: Matrix) = Ray(transformation * origin, transformation * direction)
}