use crate::ivec2::IVec2;


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
    pub position: IVec2,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(x: i32, y: i32) -> Self {
        let piece_type: PieceType = PieceType::TShaped;
        Piece {
            position: IVec2::new(x, y),
            piece_type,
        }
    }

    pub fn proyection_left(&self) -> IVec2 {
        IVec2::new(self.position.x - 1, self.position.y)
    }
    pub fn proyection_right(&self) -> IVec2 {
        IVec2::new(self.position.x + 1, self.position.y)
    }
    pub fn proyection_down(&self) -> IVec2 {
        IVec2::new(self.position.x, self.position.y + 1)
    }

    pub fn set_position(&mut self, new_possition: IVec2) {
        self.position = new_possition;
    }
}
