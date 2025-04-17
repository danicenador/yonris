use crate::ivec2::IVec2;
use crate::block::Block;


#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum PieceType {
    Straigth,
    Square,
    TShaped,
    LShapedMirror,
    LShaped,
    Skew,
    SkewMirror,
}

pub struct Piece {
    pub core_position: IVec2,
    pub blocks: Vec<Block>,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(core_position: IVec2, blocks: Vec<Block>, piece_type: PieceType) -> Self {
        Piece {
            core_position,
            blocks,
            piece_type,
        }
    }

    pub fn set_position(&mut self, new_possition: IVec2) {
        self.core_position = new_possition;
    }
}
