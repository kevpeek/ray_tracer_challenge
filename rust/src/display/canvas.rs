use crate::display::color::Color;
use crate::display::resolution::Resolution;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(resolution: Resolution) -> Canvas {
        let width = resolution.hsize();
        let height = resolution.vsize();
        Canvas {
            width,
            height,
            pixels: vec![Color::BLACK; width * height],
        }
    }

    pub fn rows(&self) -> Vec<Vec<Color>> {
        (0..self.height)
            .map(|y| (0..self.width).map(|x| self.pixel_at(x, y)).collect())
            .collect()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.index_for(x, y);
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.index_for(x, y)]
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        if x > self.width || y > self.height {
            panic!(
                "coordinate ({}, {}) is lies outside canvas size: {}x{}",
                x, y, self.width, self.height
            );
        }
        y * self.width + x
    }
}
