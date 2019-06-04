package exercises

import display.Canvas
import display.Color
import display.canvasToPpm
import display.writePpm
import geometry.Matrix
import geometry.Point
import geometry.rotationY

val width = 200
val height = 200

/**
 * Creates an image showing a dot at each hour of a clock.
 */
fun main() {
    val canvas = Canvas(width, height)

    val noon = Point(0, 0, 1)
    val oneHourRotation = rotationY(Math.PI / 6)

    val hours = (0..11).map { count -> repeated(count, oneHourRotation) }.map { it * noon }

    hours.forEach { hour ->
        val (x, y) = pointToCoordinate(hour)
        canvas.writePixel(x, y, Color.GREEN)
    }

    writePpm(canvasToPpm(canvas))
}

/**
 * Create a transformation that is the supplied transformation repeated number number.
 *
 * Repeating a transformation zero times yields the Identity Matrix.
 */
private tailrec fun repeated(number: Int, transform: Matrix, currentTransform: Matrix = Matrix.identity(4)): Matrix = when (number) {
    0 -> currentTransform
    else -> repeated(number - 1, transform, currentTransform.then(transform))
}

/**
 * Convert a Point to the x,y coordinates used by the canvas.
 */
private fun pointToCoordinate(point: Point): Pair<Int, Int> {
    val xOffset = (point.x * width / 2 * .75).toInt()
    val yOffset = (point.z * height / 2 * .75).toInt()

    val xCoordinate = xOffset + width / 2
    val yCoordinate = yOffset + height / 2

    return xCoordinate to height - yCoordinate
}