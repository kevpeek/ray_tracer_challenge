package tracing

import geometry.Matrix
import geometry.Point
import geometry.Vector
import kotlin.math.sqrt

class TransformedShape(private val delegate: Shape, val transform: Matrix): Shape {
    override fun material(): Material {
        return delegate.material()
    }

    override fun intersects(ray: Ray): List<Intersection> {
        val transformedRay = ray.transform(transform.inverse())
        // Delegate will return intersections referencing the delegate instead of this.
        // Rewrite the intersections to reference this.
        return delegate.intersects(transformedRay).map { Intersection(it.time, this) }
    }

    override fun normalAt(point: Point): Vector {
        val transformToObjectSpace = transform.inverse()
        val pointInObjectSpace = transformToObjectSpace * point
        val normalInObjectSpace = delegate.normalAt(pointInObjectSpace)
        val transformToWorldSpace = transform.submatrix(3, 3).inverse().transpose()
        return (transformToWorldSpace * normalInObjectSpace).normalize()
    }
}
