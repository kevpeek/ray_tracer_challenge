package tracing

import geometry.Matrix
import geometry.Point
import geometry.Vector
import geometry.WORLD_ORIGIN

/**
 * Representation of a Sphere.
 */
class Sphere(
    val transform: Matrix = Matrix.identity(4),
    val origin: Point = WORLD_ORIGIN,
    val material: Material = Material.DEFAULT
) {
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
