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
}