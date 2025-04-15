use crate::constants::{PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH};

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
