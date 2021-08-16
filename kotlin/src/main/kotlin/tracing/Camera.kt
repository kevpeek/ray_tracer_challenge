package tracing

import display.Canvas
import display.Resolution
import geometry.Matrix
import geometry.Point
import geometry.WORLD_ORIGIN
import kotlin.math.tan

class Camera(val resolution: Resolution, val fieldOfView: Double, val transform: Matrix = Matrix.identity(4)) {
    private val halfWidth = calculateHalfWidth()
    private val halfHeight = calculateHalfHeight()

    val pixelSize = halfWidth * 2 / resolution.hsize

    /**
     * Produce the image of the world as seen from this camera.
     */
    fun render(world: World): Canvas {
        val canvas = Canvas(Resolution(resolution.hsize, resolution.vsize))
            resolution.enumerate()
            .parallelStream()
            .map { (x, y) -> Triple(x, y, rayForPixel(x, y)) }
            .map { (x, y, ray) -> Triple(x, y, world.colorAt(ray)) }
            .forEach { (x, y, color) -> canvas.writePixel(x, y, color) }

        return canvas
    }

    /**
     * Determine the Ray going from the camera to the specified pixel.
     */
    fun rayForPixel(x: Int, y: Int): Ray {
        // the offset from the edge of the canas to the pixel's center
        val xOffset = (x + 0.5) * pixelSize
        val yOffset = (y + 0.5) * pixelSize

        // the untransformed coordinates of the pixel in world space
        // (remember that the camera looks toward -z, so +x is to the left)
        val worldX = halfWidth - xOffset
        val worldY = halfHeight - yOffset

        // using the camera matrix, transform the canvas point and the origin
        // and then compute the ray's direction vector
        // remember that the canvas is at z=-1
        val pixel = transform.inverse() * Point(worldX, worldY, -1)
        val origin = transform.inverse() * WORLD_ORIGIN
        val direction = (pixel - origin).normalize()

        return Ray(origin, direction)
    }

    private fun calculateHalfHeight(): Double {
        val halfView = tan(fieldOfView / 2)
        val aspect = resolution.aspect()

        return if (aspect >= 1) halfView / aspect else halfView
    }

    private fun calculateHalfWidth(): Double {
        val halfView = tan(fieldOfView / 2)
        val aspect = resolution.aspect()

        return if (aspect >= 1) halfView else halfView * aspect
    }
}
