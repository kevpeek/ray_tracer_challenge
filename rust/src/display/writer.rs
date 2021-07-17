use std::fs::File;
use std::io::Write;
use crate::display::color::Color;
use crate::display::canvas::Canvas;

pub fn write_canvas(canvas: &Canvas) -> std::io::Result<()> {
    write_ppm(canvas_to_ppm(canvas))
}

fn write_ppm(ppm: String) -> std::io::Result<()> {
    let mut file = File::create("output.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}

fn canvas_to_ppm(canvas: &Canvas) -> String {
    let header = generate_header(canvas);
    let body = canvas.rows().iter()
        .map(| row | convert_line(row))
        .collect::<Vec<String>>().join("\n");
    let footer = generate_footer();

    format!("{}{}{}", header, body, footer)
}

fn generate_header(canvas: &Canvas) -> String {
    format!("P3\n{} {}\n255\n", canvas.width, canvas.height)
}

fn generate_footer() -> String {
    String::from("\n")
}

/**
//  * Convert each row of the canvas into the expected output format.
//  */
fn convert_line(row: &[Color]) -> String {
    limit_width(row.iter()
        .map(| color | color.to255())
        .map(|(r, g, b)| format!("{} {} {}", r, g, b))
        .collect::<Vec<String>>().join(" "))
}

/**
 * Limit row width to 70 characters. If the width is larger, find a whitespace to split on to create a (head|rest).
 * Head will be less than 70 chars, so recursively limit rest and then combine the results.
 */
fn limit_width(row: String) -> String {
    if row.len() < 70 {
        return row;
    }

    let good_starting_point = 65;
    let index_of_space = good_starting_point + row.as_str()[good_starting_point..].find(' ').unwrap();
    let start = &row[..index_of_space];
    let rest = &row[index_of_space..];
    format!("{}\n{}", start, limit_width(String::from(rest)))
}
