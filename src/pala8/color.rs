use image::Rgba;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32, // red, from 0.0 to 1.0
    pub g: f32, // green, from 0.0 to 1.0
    pub b: f32, // blue, from 0.0 to 1.0
    pub a: f32, // alpha, from 0.0 to 1.0
}

impl Color {
    pub fn from(pixel: Rgba<u8>) -> Self {
        Color {
            r: pixel[0] as f32 / 255.0, // Normalize red channel
            g: pixel[1] as f32 / 255.0, // Normalize green channel
            b: pixel[2] as f32 / 255.0, // Normalize blue channel
            a: pixel[3] as f32 / 255.0, // Normalize alpha channel
        }
    }
}
