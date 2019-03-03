data class Canvas(val width: Int, val height: Int) {

    private val pixels = MutableList(width * height) { Color(0.0, 0.0, 0.0) }

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
    private fun indexFor(x: Int, y: Int) = y * width + x
}