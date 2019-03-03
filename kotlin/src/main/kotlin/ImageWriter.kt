
fun canvasToPpm(canvas: Canvas): String {
    val header = generateHeader(canvas)
    val body = canvas.rows().joinToString("\n") { row -> convertLine(row) }
    val footer = generateFooter()

    return header + body + footer
}

private fun generateHeader(canvas: Canvas): String {
    return """
P3
${canvas.width} ${canvas.height}
255


""".trimIndent()
}


private fun generateFooter() = "\n"
/**
 * Convert each row of the canvas into the expected output format.
 */
private fun convertLine(row: List<Color>): String {
    return limitWidth(row.joinToString(" ") { color -> color.as255().joinToString(" ") })
}

/**
 * Limit row width to 70 characters. If the width is larger, find a whitespace to split on to create a (head|rest).
 * Head will be less than 70 chars, so recursively limit rest and then combine the results.
 */
private fun limitWidth(row: String): String {
    if(row.length < 70) return row

    val indexOfSpace = row.indexOf(" ", 65)
    val start = row.substring(0 until indexOfSpace)
    val rest = row.substring(indexOfSpace + 1 until row.length)
    return start + "\n" + limitWidth(rest)
}