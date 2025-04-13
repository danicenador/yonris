use crate::constants::{PLAYFIELD_BLOCK_PX, PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH};
use crate::pala8::constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use crate::pala8::display_engine::center_line;
use crate::pala8::vec2::Vec2;

pub struct Playfield {
    pub width: i32,
    pub height: i32,
    pub top_left_corner: Vec2,
    pub block_size: i32,
}

impl Playfield {
    pub fn new() -> Self {
        let width: i32 = PLAYFIELD_WIDTH;
        let height: i32 = PLAYFIELD_HEIGHT;
        let pixel_width: i32 = PLAYFIELD_BLOCK_PX * width;
        let pixel_height: i32 = PLAYFIELD_BLOCK_PX * height;
        let top_left_corner = Vec2::new(
            center_line(pixel_width, RESOLUTION_WIDTH),
            center_line(pixel_height, RESOLUTION_HEIGHT),
        );
        Playfield {
            width,
            height,
            top_left_corner,
            block_size: PLAYFIELD_BLOCK_PX,
        }
    }
}
