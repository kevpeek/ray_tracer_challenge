package tracing

import geometry.Matrix
import geometry.Point
import geometry.Vector
import geometry.rotationZ
import geometry.scaling
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

    @Test
    fun `tThe normal on a sphere at a point on the x axis`() {
        val sphere = Sphere()
        val normal = sphere.normalAt(Point(1, 0, 0))

        assertEquals(Vector(1, 0, 0), normal)
    }

    @Test
    fun `The normal on a sphere at a point on the y axis`() {
        val sphere = Sphere()
        val normal = sphere.normalAt(Point(0, 1, 0))

        assertEquals(Vector(0, 1, 0), normal)
    }

    @Test
    fun `The normal on a sphere at a point on the z axis`() {
        val sphere = Sphere()
        val normal = sphere.normalAt(Point(0, 0, 1))

        assertEquals(Vector(0, 0, 1), normal)
    }

    @Test
    fun `The normal on a sphere at a nonaxial point`() {
        val sphere = Sphere()
        val normal = sphere.normalAt(Point(Math.sqrt(3.0) / 3, Math.sqrt(3.0) / 3, Math.sqrt(3.0) / 3))

        assertEquals(Vector(Math.sqrt(3.0) / 3, Math.sqrt(3.0) / 3, Math.sqrt(3.0) / 3), normal)
    }

    @Test
    fun `The normal is a normalized vector`() {
        val sphere = Sphere()
        val normal = sphere.normalAt(Point(Math.sqrt(3.0) / 3, Math.sqrt(3.0) / 3, Math.sqrt(3.0) / 3))
        assertEquals(normal.normalize(), normal)
    }

    @Test
    fun `Computing the normal on a translated sphere`() {
        val sphere = Sphere(translation(0, 1, 0))

        val normal = sphere.normalAt(Point(0, 1.70711, -0.70711))
        assertEquals(Vector(0, 0.70711, -0.70711), normal)
    }

    @Test
    fun `Computing the normal on a transformed sphere`() {
        val sphere = Sphere(scaling(1, 0.5, 1) * rotationZ(Math.PI / 5))

        val normal = sphere.normalAt(Point(0, Math.sqrt(2.0) / 2, -Math.sqrt(2.0) / 2))
        assertEquals(Vector(0, 0.97014, -0.24254), normal)
    }

    @Test
    fun `A sphere has a default material`() {
        val sphere = Sphere()

        assertEquals(Material.DEFAULT, sphere.material)
    }

    @Test
    fun `A sphere may be assigned a material`() {
        val material = Material()
        val sphere = Sphere(material = material)

        assertEquals(material, sphere.material)
    }
}