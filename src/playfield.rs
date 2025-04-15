use crate::constants::{PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH};
use crate::piece::Piece;
use crate::ivec2::IVec2;

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

    pub fn move_piece_left(&mut self) {
        let new_possition: IVec2 = self.piece.proyection_left();
        if self.is_valid_position(&new_possition) {
            self.piece.set_position(new_possition);
        }
    }

    pub fn move_piece_right(&mut self) {
        let new_possition: IVec2 = self.piece.proyection_right();
        if self.is_valid_position(&new_possition) {
            self.piece.set_position(new_possition);
        }
    }

    pub fn move_piece_down(&mut self) {
        let new_possition: IVec2 = self.piece.proyection_down();
        if self.is_valid_position(&new_possition) {
            self.piece.set_position(new_possition);
        }
    }

    pub fn is_valid_position(&self, new_possition: &IVec2) -> bool {
        if new_possition.x <= 0 || new_possition.x > self.width {
            return false;
        }
        if new_possition.y <= 0 || new_possition.y > self.height {
            return false;
        }
        true
    }
}
