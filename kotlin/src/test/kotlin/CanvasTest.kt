import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class CanvasTest {

    @Test
    fun `Creating a canvas`() {
        val canvas = Canvas(10, 20)
        assertEquals(10, canvas.width)
        assertEquals(20, canvas.height)

        val allPixels = (0 until 10).flatMap { x -> (0 until 20).map { y -> x to y } }
        allPixels.forEach { (x, y) -> assertEquals(Color(0.0, 0.0, 0.0), canvas.pixelAt(x, y)) }
    }

    @Test
    fun `Writing pixels to a canvas`() {
        val canvas = Canvas(10, 20)
        canvas.writePixel(2, 3, Color(1.0, 0.0, 0.0))
        assertEquals(Color(1.0, 0.0, 0.0), canvas.pixelAt(2, 3))
    }
}