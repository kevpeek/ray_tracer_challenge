
fun canvasToPpm(canvas: Canvas): String {
    val header = generateHeader(canvas)
    val lines = canvas.rows().joinToString("\n") { row -> convertLine(row) }

    return header + lines
}

private fun generateHeader(canvas: Canvas): String {
    return """
P3
${canvas.width} ${canvas.height}
255

""".trimIndent()
}

/**
 * Convert each row of the canvas into the expected output format.
 */
private fun convertLine(row: List<Color>): String {
    return row.joinToString(" ") { color -> color.as255().joinToString(" ") }
}