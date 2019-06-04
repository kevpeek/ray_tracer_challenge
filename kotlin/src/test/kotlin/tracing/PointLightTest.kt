package tracing

import display.Color
import geometry.Point
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PointLightTest {

    @Test
    fun `A point light has a position and intensity`() {
        val intensity = Color(1, 1, 1)
        val position = Point(0, 0, 0)
        val light = PointLight(position, intensity)

        assertEquals(position, light.position)
        assertEquals(intensity, light.intensity)
    }
}