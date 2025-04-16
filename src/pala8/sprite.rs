use crate::pala8::color::Color;
use image::{DynamicImage, GenericImageView, Rgba};

pub struct Sprite {
    pixel_grid: Vec<Vec<Color>>,
}
impl Sprite {

    pub fn new(pixel_grid: Vec<Vec<Color>>) -> Self{
        Sprite { pixel_grid }
    }

    pub fn load(sprite_path: &str) -> Self {
        let img: DynamicImage = image::open(sprite_path).expect("Failed to open sprite image.");

        let mut pixel_grid: Vec<Vec<Color>> = Vec::new();
        for row in 0..img.height() {
            let mut pixel_row: Vec<Color> = Vec::new();
            for col in 0..img.width() {
                let pixel: Rgba<u8> = img.get_pixel(col, row);
                pixel_row.push(Color::from(pixel));
            }
            pixel_grid.push(pixel_row);
        }

        Sprite::new(pixel_grid)
    }

    pub fn slice(
        &self,
        start_x: usize,
        start_y: usize,
        slice_width: usize,
        slice_height: usize,
    ) -> Self {
        let mut sliced_grid: Vec<Vec<Color>> = Vec::new();
        for row_offset in 0..slice_height {
            let mut row: Vec<Color> = Vec::new();
            for col_offset in 0..slice_width {
                if start_y + row_offset < self.pixel_grid.len()
                    && start_x + col_offset < self.pixel_grid[start_y + row_offset].len()
                {
                    row.push(self.pixel_grid[start_y + row_offset][start_x + col_offset]);
                } else {
                    row.push(Color::from(Rgba([0, 0, 0, 0])));
                }
            }
            sliced_grid.push(row);
        }
        Sprite::new(sliced_grid)
    }

    pub fn get_pixel(&self, col: usize, row: usize) -> Color {
        if row < self.pixel_grid.len() && col < self.pixel_grid[row].len() {
            self.pixel_grid[row][col]
        } else {
            Color::from(Rgba([0, 0, 0, 0])) // Return transparent black if out of bounds
        }
    }

    pub fn get_width(&self) -> usize {
        if self.pixel_grid.is_empty() {
            0
        } else {
            self.pixel_grid[0].len()
        }
    }

    pub fn get_height(&self) -> usize {
        self.pixel_grid.len()
    }
}
