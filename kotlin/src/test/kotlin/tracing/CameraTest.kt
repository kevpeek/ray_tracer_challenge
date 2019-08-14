package tracing

import display.Color
import geometry.Matrix
import geometry.Point
import geometry.Vector
import geometry.WORLD_ORIGIN
import geometry.rotationY
import geometry.translation
import geometry.viewTransform
import helper.EPSILON
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import kotlin.math.sqrt

class CameraTest {

    @Test
    fun `Constructing a camera`() {
        val hsize = 160
        val vsize = 120
        val fieldOfView = Math.PI / 2

        val camera = Camera(hsize, vsize, fieldOfView)

        assertEquals(160, camera.hsize)
        assertEquals(120, camera.vsize)
        assertEquals(Math.PI / 2, camera.fieldOfView)
        assertEquals(Matrix.identity(4), camera.transform)
    }

    @Test
    fun `The pixel size for a horizontal canvas`() {
        val camera = Camera(200, 125, Math.PI / 2)
        assertEquals(0.01, camera.pixelSize, EPSILON)
    }

    @Test
    fun `The pixel size for a vertical canvas`() {
        val camera = Camera(125, 200, Math.PI / 2)
        assertEquals(0.01, camera.pixelSize, EPSILON)
    }

    @Test
    fun `Constructing a ray through the center of the canvas`() {
        val camera = Camera(201, 101, Math.PI / 2)

        val ray = camera.rayForPixel(100, 50)

        assertEquals(WORLD_ORIGIN, ray.origin)
        assertEquals(Vector(0, 0, -1), ray.direction)
    }

    @Test
    fun `Constructing a ray through a corner of the canvas`() {
        val camera = Camera(201, 101, Math.PI / 2)

        val ray = camera.rayForPixel(0, 0)

        assertEquals(WORLD_ORIGIN, ray.origin)
        assertEquals(Vector(0.66519, 0.33259, -0.66851), ray.direction)
    }

    @Test
    fun `Constructing a ray when the camera is transformed`() {
        val transform = translation(0, -2, 5).then(rotationY(Math.PI / 4))
        val camera = Camera(201, 101, Math.PI / 2, transform)

        val ray = camera.rayForPixel(100, 50)

        assertEquals(Point(0, 2, -5), ray.origin)
        assertEquals(Vector(sqrt(2.0) / 2, 0, -sqrt(2.0) / 2), ray.direction)
    }

    @Test
    fun `Rendering a world with a camera`() {
        val world = World.default()

        val from = Point(0, 0, -5)
        val to = WORLD_ORIGIN
        val up = Vector(0, 1, 0)
        val camera = Camera(11, 11, Math.PI / 2, viewTransform(from, to, up))

        val image = camera.render(world)
        assertEquals(Color(0.38066, 0.47583, 0.2855), image.pixelAt(5, 5))
    }
}