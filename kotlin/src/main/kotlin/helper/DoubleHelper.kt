package helper

private const val EPSILON = .00001
fun almost(a: Double, b: Double) = Math.abs(a - b) < EPSILON

infix fun Double.approximately(other: Double) = almost(this, other)