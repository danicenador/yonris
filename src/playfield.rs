use crate::constants::{PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH};
use crate::piece::Piece;

const STARTING_X: i32 = 6;
const STARTING_Y: i32 = 1;

pub struct Playfield {
    pub width: i32,
    pub height: i32,
    pub piece: Piece,
}

impl Playfield {
    pub fn new() -> Self {
        let width: i32 = PLAYFIELD_WIDTH;
        let height: i32 = PLAYFIELD_HEIGHT;
        let piece: Piece = Piece::new(STARTING_X, STARTING_Y);
        Playfield {
            width,
            height,
            piece,
        }
    }
}
