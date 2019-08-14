package exercises

import display.Color
import display.canvasToPpm
import display.writePpm
import geometry.Point
import geometry.Vector
import geometry.WORLD_ORIGIN
import geometry.rotationX
import geometry.rotationY
import geometry.scaling
import geometry.translation
import geometry.viewTransform
import tracing.Camera
import tracing.Material
import tracing.PointLight
import tracing.Sphere
import tracing.World

fun main() {

    val floorTransform = scaling(10, 0.01, 10)
    val floorMaterial = Material(Color(1, 0.9, 0.9), specular = 0.0)
    val floor = Sphere(floorTransform, WORLD_ORIGIN, floorMaterial)

    val leftWallTransform = scaling(10, 0.01, 10)
        .then(rotationX(Math.PI / 2))
        .then(rotationY(-Math.PI / 4))
        .then(translation(0, 0, 5))
    val leftWall = Sphere(leftWallTransform, WORLD_ORIGIN, floorMaterial)

    val rightWallTransform = scaling(10, 0.01, 10)
        .then(rotationX(Math.PI / 2))
        .then(rotationY(Math.PI / 4))
        .then(translation(0, 0, 5))
    val rightWall = Sphere(rightWallTransform, WORLD_ORIGIN, floorMaterial)

    val middleTransform = translation(-0.5, 1, 0.5)
    val middleMaterial = Material(Color(0.1, 1, 0.5), diffuse = 0.7, specular = 0.3)
    val middle = Sphere(middleTransform, WORLD_ORIGIN, middleMaterial)

    val rightTransform = scaling(0.5, 0.5, 0.5).then(translation(1.5, 0.5, -0.5))
    val right = Sphere(rightTransform, WORLD_ORIGIN, middleMaterial)

    val leftTransform = scaling(0.33, 0.33, 0.33).then(translation(-1.5, 0.33, -0.75))
    val leftMaterial = Material(Color(1, 0.8, 0.1), diffuse = 0.7, specular = 0.3)
    val left = Sphere(leftTransform, WORLD_ORIGIN, leftMaterial)

    val lightSource = PointLight(Point(-10, 10, -10), Color(1, 1, 1))
    val world = World(listOf(floor, leftWall, rightWall, middle, right, left), lightSource)

    val cameraTransform = viewTransform(Point(0, 1.5, -5), Point(0, 1, 0), Vector(0, 1, 0))
    val camera = Camera(800, 400, Math.PI / 3, cameraTransform)

    val canvas = camera.render(world)
    writePpm(canvasToPpm(canvas))
}