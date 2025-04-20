use crate::ivec2::IVec2;
use crate::piece::PieceType;

#[derive(Clone)]
pub struct Block {
    pub position: IVec2,
    pub piece_type: PieceType,
    pub is_core: bool,
}

impl Block {
    pub fn new(position: IVec2, piece_type: PieceType, is_core: bool) -> Self {
        Block {
            position,
            piece_type,
            is_core,
        }
    }

    pub fn projection_left(&self) -> IVec2 {
        IVec2::new(self.position.x - 1, self.position.y)
    }
    pub fn projection_right(&self) -> IVec2 {
        IVec2::new(self.position.x + 1, self.position.y)
    }
    pub fn projection_down(&self) -> IVec2 {
        IVec2::new(self.position.x, self.position.y + 1)
    }

    pub fn set_position(&mut self, new_possition: IVec2) {
        self.position = new_possition;
    }

    pub fn get_position(&self) -> &IVec2 {
        &self.position
    }
}
