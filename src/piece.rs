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
    pub blocks: Vec<Block>,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(blocks: Vec<Block>, piece_type: PieceType) -> Self {
        Piece {
            blocks,
            piece_type,
        }
    }
}
