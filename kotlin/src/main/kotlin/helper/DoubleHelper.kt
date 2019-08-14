package helper

import kotlin.math.abs

const val EPSILON = .00001
private fun almost(a: Double, b: Double) = abs(a - b) < EPSILON

infix fun Double.approximately(other: Double) = almost(this, other)