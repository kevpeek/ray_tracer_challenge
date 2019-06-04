package tracing

import display.Color
import geometry.Point
import geometry.Vector
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class MaterialTest {

    val material = Material()
    val position = Point(0, 0, 0)

    @Test
    fun `The default material`() {
        assertEquals(Color(1, 1, 1), material.color)
        assertEquals(0.1, material.ambient)
        assertEquals(0.9, material.diffuse)
        assertEquals(0.9, material.specular)
        assertEquals(200.0, material.shininess)
    }

    @Test
    fun `Lighting with the eye between the light and the surface`() {
        val eyeVector = Vector(0, 0, -1)
        val normal = Vector(0, 0, -1)
        val light = PointLight(Point(0, 0, -10), Color(1, 1, 1))

        val result = lighting(material, light, position, eyeVector, normal)
        assertEquals(Color(1.9, 1.9, 1.9), result)
    }

    @Test
    fun `Lighting with the eye between light and surface, eye offset 45°`() {
        val eyeVector = Vector(0, Math.sqrt(2.0) / 2, - Math.sqrt(2.0) / 2)
        val normal = Vector(0, 0, -1)
        val light = PointLight(Point(0, 0, -10), Color(1, 1, 1))

        val result = lighting(material, light, position, eyeVector, normal)
        assertEquals(Color(1, 1, 1), result)
    }

    @Test
    fun `Lighting with eye opposite surface, light offset 45°`() {
        val eyeVector = Vector(0, 0, -1)
        val normal = Vector(0, 0, -1)
        val light = PointLight(Point(0, 10, -10), Color(1, 1, 1))

        val result = lighting(material, light, position, eyeVector, normal)
        assertEquals(Color(0.7364, 0.7364, 0.7364), result)
    }

    @Test
    fun `Lighting with eye in the path of the reflection vector`() {
        val eyeVector = Vector(0, -Math.sqrt(2.0) / 2, -Math.sqrt(2.0) / 2)
        val normal = Vector(0, 0, -1)
        val light = PointLight(Point(0, 10, -10), Color(1, 1, 1))

        val result = lighting(material, light, position, eyeVector, normal)
        assertEquals(Color(1.6364, 1.6364, 1.6364), result)
    }

    @Test
    fun `Lighting with the light behind the surface`() {
        val eyeVector = Vector(0, 0, -1)
        val normal = Vector(0, 0, -1)
        val light = PointLight(Point(0, 0, 10), Color(1, 1, 1))

        val result = lighting(material, light, position, eyeVector, normal)
        assertEquals(Color(0.1, 0.1, 0.1), result)
    }
}