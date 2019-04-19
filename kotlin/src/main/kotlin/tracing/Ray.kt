package tracing

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
}