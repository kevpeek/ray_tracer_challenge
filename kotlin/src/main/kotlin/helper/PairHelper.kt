package helper

operator fun Iterable<Int>.times(other: Iterable<Int>) = this.flatMap { x -> other.map { y -> x to y } }
