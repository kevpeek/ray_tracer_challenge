fun main() {
    println("~~~~~Starting~~~~~")

    val gravity = Vector(0.0, -0.1, 0.0)
    val wind = Vector(-0.01, 0.0, 0.0)
    val environment = Pair(gravity, wind)
    val projectile = Pair(Point(0.0, 0.0, 0.0), Vector(1.0, 1.0, 0.0).normalize())

    iterate(projectile, environment)


    println("~~~~~Done~~~~~")
}

fun iterate(projectile: Pair<Point, Vector>, environment: Pair<Vector, Vector>) {
    val (location, velocity) = projectile
    val (gravity, wind) = environment
    println(location)

    if (location.y < 0) return

    val newLocation = location.plus(velocity)
    val newVelocity = velocity.plus(gravity).plus(wind)
    iterate(Pair(newLocation, newVelocity), environment)
}