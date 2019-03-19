import display.Canvas
import display.canvasToPpm
import display.writePpm
import helper.times


fun main() {
    println("~~~~~Starting~~~~~")

    val nums = Builder(5).of(1, 2, 3)
    nums.forEach(::println)

    println("~~~~~Done~~~~~")
}


class Builder(val size: Int) {
    fun of(vararg values: Int) = values.toList()
}