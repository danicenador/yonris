use crate::constants::{PLAYFIELD_BLOCK_PX, PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH};
use crate::pala8::constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use crate::pala8::display_engine::center_line;
use crate::pala8::vec2::Vec2;

pub struct Playfield {
    pub width: i32,
    pub height: i32,
}

impl Playfield {
    pub fn new() -> Self {
        let width: i32 = PLAYFIELD_WIDTH;
        let height: i32 = PLAYFIELD_HEIGHT;
        Playfield {
            width,
            height,
        }
    }
}
