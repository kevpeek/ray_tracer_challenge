import java.lang.RuntimeException

data class Canvas(val width: Int, val height: Int) {

    private val pixels = MutableList(width * height) { Color.BLACK }

    fun pixelAt(x: Int, y: Int) = pixels[indexFor(x, y)]

    fun writePixel(x: Int, y: Int, color: Color) {
        pixels[indexFor(x, y)] = color
    }

    /**
     * Returns a list of rows, each of which is a list of Colors.
     */
    fun rows() = (0 until height).map { y -> (0 until width).map { x -> pixelAt(x, y) }}

    /**
     * Determine the list index for the specified coordinates.
     */
    private fun indexFor(x: Int, y: Int): Int {
        if (x < 0 || x > width || y < 0 || y > height) {
            throw CanvasException("coordinate ($x, $y) is lies outside canvas size: ${width}x$height")
        }

        return y * width + x
    }

    class CanvasException(message: String): RuntimeException(message)
}