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

    pub fn move_left(&mut self) {
        self.position.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.position.x += 1;
    }

    pub fn move_down(&mut self) {
        self.position.y += 1;
    }
}
