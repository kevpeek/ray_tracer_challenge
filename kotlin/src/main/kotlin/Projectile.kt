fun main() {
    println("~~~~~Starting~~~~~")

    val gravity = Vector(0.0, -0.1, 0.0)
    val wind = Vector(-0.01, 0.0, 0.0)
    val environment = Pair(gravity, wind)
    val projectile = Pair(Point(0.0, 0.0, 0.0), Vector(1.0, 1.0, 0.0).normalize())

    val points = iterate(listOf(projectile), environment)

    val width = 200
    val height = 200
    val red = Color(1.0, 0.0, 0.0)
    val canvas = Canvas(width, height)
    points.map { it.first }
        .map { point -> Pair((point.x * 10).toInt(), (point.y * 10).toInt())}
        .map { (x, y) -> Pair(x, height - y - 1) }
        .forEach { (x, y) -> canvas.writePixel(x, y, red)}

    writePpm(canvasToPpm(canvas))

    println("~~~~~Done~~~~~")
}

fun iterate(path: List<Pair<Point, Vector>>, environment: Pair<Vector, Vector>): List<Pair<Point, Vector>> {
    val (location, velocity) = path.last()
    val (gravity, wind) = environment
    println(location)

    val newLocation = location.plus(velocity)
    if (newLocation.y < 0) return path

    val newVelocity = velocity.plus(gravity).plus(wind)
    return iterate(path.plus(Pair(newLocation, newVelocity)), environment)
}