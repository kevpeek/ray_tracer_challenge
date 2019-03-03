import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ImageWriterTest {

    @Test
    fun `Constructing the PPM header`() {
        val canvas = Canvas(5, 3)
        val ppmText = canvasToPpm(canvas)
        val ppmLines = ppmText.lines()
        assertEquals("P3", ppmLines[0])
        assertEquals("5 3", ppmLines[1])
        assertEquals("255", ppmLines[2])
    }

    @Test
    fun `Constructing the PPM pixel data`() {
        val canvas = Canvas(5, 3)
        canvas.writePixel(0, 0, Color(1.5, 0.0, 0.0))
        canvas.writePixel(2, 1, Color(0.0, 0.5, 0.0))
        canvas.writePixel(4, 2, Color(-0.5, 0.0, 1.0))

        val ppm = canvasToPpm(canvas)
        val ppmLines = ppm.lines()

        assertEquals("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", ppmLines[3])
        assertEquals("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", ppmLines[4])
        assertEquals("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", ppmLines[5])
    }

    @Test
    fun `Splitting long lines in PPM files`() {
        /*
  Given c â† canvas(10, 2)
  When every pixel of c is set to color(1, 0.8, 0.6)
    And ppm â† canvas_to_ppm(c)
  Then rows 4-7 of ppm are
    """
    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
    153 255 204 153 255 204 153 255 204 153 255 204 153
    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
    153 255 204 153 255 204 153 255 204 153 255 204 153
    """
         */
    }

}