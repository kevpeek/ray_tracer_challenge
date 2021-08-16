package display

import java.lang.RuntimeException

data class Canvas(val resolution: Resolution) {

    private val pixels = MutableList(resolution.hsize * resolution.vsize) { Color.BLACK }

    fun pixelAt(x: Int, y: Int) = pixels[indexFor(x, y)]

    fun writePixel(x: Int, y: Int, color: Color) {
        pixels[indexFor(x, y)] = color
    }

    /**
     * Returns a list of rows, each of which is a list of Colors.
     */
    fun rows() = (0 until resolution.vsize).map { y -> (0 until resolution.hsize).map { x -> pixelAt(x, y) } }

    /**
     * Determine the list index for the specified coordinates.
     */
    private fun indexFor(x: Int, y: Int): Int {
        if (x < 0 || x > resolution.hsize || y < 0 || y > resolution.vsize) {
            throw CanvasException("coordinate ($x, $y) is lies outside canvas size: ${resolution.hsize}x${resolution.vsize}")
        }

        return y * resolution.hsize + x
    }

    class CanvasException(message: String) : RuntimeException(message)
}
