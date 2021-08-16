package exercises

import display.*
import geometry.Matrix
import geometry.Point
import tracing.Ray
import tracing.Sphere
import tracing.hit

fun main() {
    val canvasPixels = 100
    val canvas = Canvas(Resolution(canvasPixels, canvasPixels))
    val transformation = Matrix.identity(4)
    val shape = Sphere().withTransform { transformation }

    val rayOrigin = Point(0, 0, -5)
    val wallZ = 10
    val wallSize = 7.0

    val pixelSize = wallSize / canvasPixels
    val half = wallSize / 2

    (0 until canvasPixels).forEach { y ->
        val worldY = half - pixelSize * y
        (0 until canvasPixels).forEach { x ->
            val worldX = -half + pixelSize * x
            val position = Point(worldX, worldY, wallZ)
            val rayDirection = (position - rayOrigin).normalize()
            val ray = Ray(rayOrigin, rayDirection)
            val intersections = shape.intersects(ray)
            intersections.hit()?.let { canvas.writePixel(x, y, Color.RED) }
        }
    }

    writePpm(canvasToPpm(canvas))
}
