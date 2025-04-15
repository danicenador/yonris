use crate::ivec2::IVec2;
use crate::pala8::color::Color;

pub struct Piece {
    pub position: IVec2,
    pub color: Color,
}

impl Piece {
    pub fn new(x: i32, y: i32) -> Self {
        let color: Color = Color {
            r: 0.0 / 128.0,
            g: 90.0 / 128.0,
            b: 80.0 / 128.0,
            a: 1.0,
        };
        Piece {
            position: IVec2::new(x, y),
            color,
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
