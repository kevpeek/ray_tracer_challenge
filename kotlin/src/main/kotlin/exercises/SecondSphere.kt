package exercises

import display.Canvas
import display.Color
import display.canvasToPpm
import display.writePpm
import geometry.Matrix
import geometry.Point
import tracing.Material
import tracing.PointLight
import tracing.Ray
import tracing.Sphere
import tracing.hit
import tracing.intersects
import tracing.lighting

private fun getShape(): Sphere {
    val material = Material(color = Color(1, 0.2, 1))
    val transformation = Matrix.identity(4)
    return Sphere(transform = transformation, material = material)
}

private fun getLight(): PointLight {
    val lightPosition = Point(-10, 10, -10)
    val lightColor = Color.WHITE
    return PointLight(lightPosition, lightColor)
}

fun main() {
    val shape = getShape()
    val light = getLight()

    val canvasPixels = 100
    val canvas = Canvas(canvasPixels, canvasPixels)
    val rayOrigin = Point(0, 0, -5)
    val wallZ = 10
    val wallSize = 7.0
    val pixelSize = wallSize / canvasPixels
    val half = wallSize / 2

    getCanvasPoints(canvasPixels).forEach { (x, y) ->
        val worldY = half - pixelSize * y
        val worldX = -half + pixelSize * x
        val position = Point(worldX, worldY, wallZ)
        val rayDirection = (position - rayOrigin).normalize()
        val ray = Ray(rayOrigin, rayDirection)
        val intersections = intersects(shape, ray)
        hit(intersections)?.let { hit ->
            val point = ray.position(hit.time)
            val normal = hit.thing.normalAt(point)
            val eye = -(ray.direction)
            val color = lighting(shape.material, light, point, eye, normal)
            canvas.writePixel(x, y, color)
        }
    }

    writePpm(canvasToPpm(canvas))
}

private fun getCanvasPoints(canvasSize: Int) = (0 until canvasSize).flatMap { y -> (0 until canvasSize).map { x -> x to y } }