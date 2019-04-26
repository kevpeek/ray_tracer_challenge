package tracing

import geometry.Matrix
import geometry.WORLD_ORIGIN
import geometry.Point
import geometry.Vector

/**
 * Representation of a Sphere.
 */
class Sphere(val transform: Matrix = Matrix.identity(4)) {

    /**
     * The center of the sphere.
     */
    val origin = WORLD_ORIGIN

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
}