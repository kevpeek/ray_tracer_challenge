package display

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Test

class CanvasTest {

    @Test
    fun `Creating a canvas`() {
        val canvas = Canvas(10, 20)
        assertEquals(10, canvas.width)
        assertEquals(20, canvas.height)

        val allPixels = (0 until 10).flatMap { x -> (0 until 20).map { y -> x to y } }
        allPixels.forEach { (x, y) -> assertEquals(Color.BLACK, canvas.pixelAt(x, y)) }
    }

    @Test
    fun `Writing pixels to a canvas`() {
        val canvas = Canvas(10, 20)
        canvas.writePixel(2, 3, Color.RED)
        assertEquals(Color.RED, canvas.pixelAt(2, 3))
    }

    @Test
    fun `writing outside the canvas throws exception`() {
        val canvas = Canvas(10, 10)
        assertThrows(Canvas.CanvasException::class.java) { canvas.writePixel(11, 11, Color.BLACK) }
    }

    @Test
    fun `reading outside the canvas throws exception`() {
        val canvas = Canvas(10, 10)
        assertThrows(Canvas.CanvasException::class.java) { canvas.pixelAt(11, 11) }
    }

    @Test
    fun `rows with empty canvas`() {
        val canvas = Canvas(0, 0)
        assertEquals(emptyList<List<Color>>(), canvas.rows())
    }

    @Test
    fun `rows properly structures data`() {
        val canvas = Canvas(2, 3)
        val rows = canvas.rows()
        assertEquals(3, rows.size)
        rows.forEach { assertEquals(2, it.size) }
    }
}
