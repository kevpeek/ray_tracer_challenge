use crate::display::canvas::Canvas;

mod geometry;
mod display;
mod helper;

fn main() {
    let canvas = Canvas::checker_square(200);
    display::writer::write_canvas(&canvas).unwrap();
}
