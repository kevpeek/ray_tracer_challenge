fun main() {
    println("~~~~~Starting~~~~~")

    val canvas = Canvas(100, 100)
    writePpm(canvasToPpm(canvas))

    println("~~~~~Done~~~~~")
}