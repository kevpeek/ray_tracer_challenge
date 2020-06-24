package display

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ColorTest {

    @Test
    fun `Colors are (red, green, blue) tuples`() {
        val color = Color(-0.5, 0.4, 1.7)
        assertEquals(-0.5, color.red)
        assertEquals(0.4, color.green)
        assertEquals(1.7, color.blue)
    }

    @Test
    fun `Adding colors`() {
        val color1 = Color(0.9, 0.6, 0.75)
        val color2 = Color(0.7, 0.1, 0.25)
        assertEquals(Color(1.6, 0.7, 1.0), color1 + color2)
    }

    @Test
    fun `Subtracting colors`() {
        val color1 = Color(0.9, 0.6, 0.75)
        val color2 = Color(0.7, 0.1, 0.25)
        assertEquals(Color(0.2, 0.5, 0.5), color1 - color2)
    }

    @Test
    fun `Multiplying a color by a scalar`() {
        val color = Color(0.2, 0.3, 0.4)
        assertEquals(Color(0.4, 0.6, 0.8), color * 2.0)
    }

    @Test
    fun `Multiplying colors`() {
        val color1 = Color(1.0, 0.2, 0.4)
        val color2 = Color(0.9, 1.0, 0.1)
        assertEquals(Color(0.9, 0.2, 0.04), color1 * color2)
    }
}
