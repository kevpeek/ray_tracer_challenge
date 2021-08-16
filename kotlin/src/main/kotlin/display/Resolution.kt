package display

class Resolution(val hsize: Int, val vsize: Int) {

    fun aspect(): Double = hsize.toDouble() / vsize

    fun enumerate(): List<Pair<Int, Int>> = (0 until vsize).flatMap { y -> (0 until hsize).map { x -> x to y } }

}
