use raylib::prelude::*;

#[derive(Clone)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    buffer: Vec<Vec<Color>>,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let row = vec![Color::BLACK; width as usize];
        let buffer = vec![row; height as usize];
        Self {
            width,
            height,
            buffer,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.current_color = color;
        for y in 0..self.height {
            for x in 0..self.width {
                self.buffer[y as usize][x as usize] = color;
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.buffer[y as usize][x as usize] = self.current_color;
        }
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            self.buffer[y as usize][x as usize]
        } else {
            Color::BLACK
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_image(&self) -> Image {
        let mut image = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
        for y in 0..self.height {
            for x in 0..self.width {
                image.draw_pixel(x as i32, y as i32, self.buffer[y as usize][x as usize]);
            }
        }
        image
    }
}
