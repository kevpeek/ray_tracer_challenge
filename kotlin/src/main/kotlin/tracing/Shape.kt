package tracing

import geometry.Matrix
import geometry.Point
import geometry.Vector

interface Shape {

    fun withTransform(otherTransform: () -> Matrix): TransformedShape {
        return TransformedShape(this, otherTransform())
    }

    fun material(): Material

    /**
     * Returns the list of Intersections between the ray and sphere.
     */
    fun intersects(ray: Ray): List<Intersection>

    /**
     * Return the Vector normal to this sphere at the supplied point.
     */
    fun normalAt(point: Point): Vector
}
