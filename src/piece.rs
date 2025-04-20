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

    pub fn projection_rotation(&self) -> Vec<IVec2> {
        let core_position = self.get_core_position();

        let mut projections = Vec::new();
        for block in &self.blocks {
            let block_direction: IVec2 = block.position.sub(core_position);
            let rotated_direction = IVec2::new(-block_direction.y, block_direction.x);
            projections.push(core_position.add(&rotated_direction));
        }
        projections
    }

    fn get_core_position(&self) -> &IVec2 {
        for block in &self.blocks {
            if block.is_core {
                return block.get_position();
            }
        }
        panic!("No core block found in the piece.");
    }

    pub fn set_block_possitions(&mut self, new_possitions: Vec<IVec2>) {
        for (block, new_possition) in self.blocks.iter_mut().zip(new_possitions) {
            block.set_position(new_possition);
        }
    }
}
