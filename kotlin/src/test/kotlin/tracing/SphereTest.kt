package tracing

import geometry.Matrix
import geometry.translation
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.fail

class SphereTest {

    @Test
    fun `A sphere's default transformation`() {
        val sphere = Sphere()
        assertEquals(Matrix.identity(4), sphere.transform)
    }

    @Test
    fun `Changing a sphere's transformation`() {
        val transformation = translation(2, 3, 4)
        val sphere = Sphere(transformation)
        assertEquals(transformation, sphere.transform)
    }

}