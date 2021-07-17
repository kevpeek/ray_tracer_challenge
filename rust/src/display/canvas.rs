use crate::display::color::Color;
use crate::helper::enumerate_coordinates;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {width, height, pixels: vec![Color::BLACK; width * height] }
    }

    pub fn checker_square(size: usize) -> Canvas {
        let mut canvas = Canvas::new(size, size);
        enumerate_coordinates(0..size, 0..size).into_iter().map(|(x,y)| match x % 2 == y % 2 {
            false => (x, y, Color::GREEN),
            true => (x, y, Color::BLUE),
        }).for_each(|(x, y, color)| canvas.write_pixel(x, y, color));

        canvas
    }

    pub fn rows(&self) -> Vec<Vec<Color>> {
        (0..self.height).map(|y| {
            (0..self.width).map(|x| self.pixel_at(x, y)).collect()
        }).collect()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.index_for(x, y);
        self.pixels[index] = color;
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.index_for(x, y)]
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        if x > self.width || y > self.height {
            panic!("coordinate ({}, {}) is lies outside canvas size: {}x{}", x, y, self.width, self.height);
        }
        y * self.width + x
    }
}
