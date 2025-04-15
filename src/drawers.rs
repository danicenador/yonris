use crate::playfield;
use crate::pala8::vec2::Vec2;
use crate::pala8::display_engine::center_line;

use crate::constants::PLAYFIELD_BLOCK_PX;
use crate::pala8::constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};

pub struct PlayfieldDrawer<'a> {
    pub playfield: &'a playfield::Playfield,
    pub top_left_pixel: Vec2,
}

impl<'a> PlayfieldDrawer<'a> {
    pub fn new(playfield: &'a playfield::Playfield) -> Self {
        let pixel_width: i32 = PLAYFIELD_BLOCK_PX * playfield.width;
        let pixel_height: i32 = PLAYFIELD_BLOCK_PX * playfield.height;
        let top_left_pixel: Vec2 = Vec2::new(
            center_line(pixel_width, RESOLUTION_WIDTH),
            center_line(pixel_height, RESOLUTION_HEIGHT),
        );

        PlayfieldDrawer {
            playfield,
            top_left_pixel,
        }
    }
}
