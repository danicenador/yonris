use crate::ivec2::IVec2;
use crate::block::Block;
use crate::piece::{Piece, PieceType};


pub struct PieceFactory {
    piece_types: Vec<PieceType>,
}

impl PieceFactory {
    pub fn new() -> Self {
        let piece_types = vec![
            PieceType::Straigth,
            PieceType::Square,
            PieceType::TShaped,
            PieceType::LShapedMirror,
            PieceType::LShaped,
            PieceType::Skew,
            PieceType::SkewMirror,
        ];

        PieceFactory {
            piece_types,
        }
    }

    pub fn get(&self, spawn_position: IVec2) -> Piece {

        let random_number: u32 = rand::random::<u32>();
        let piece_choice: u32 = random_number % self.piece_types.len() as u32;

        let piece_type = self.piece_types[piece_choice as usize].clone();

        let blocks = match piece_type {
            PieceType::Straigth => vec![
                Block::new(spawn_position.clone(), piece_type.clone(), true),
                Block::new(spawn_position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(2, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(-1, 0)), piece_type.clone(), false),
            ],
            PieceType::Square => vec![
                Block::new(spawn_position.clone(), piece_type.clone(), true),
                Block::new(spawn_position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(0, 1)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(1, 1)), piece_type.clone(), false),
            ],
            PieceType::TShaped => vec![
                Block::new(spawn_position.clone(), piece_type.clone(), true),
                Block::new(spawn_position.add(&IVec2::new(-1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(0, 1)), piece_type.clone(), false),
            ],
            PieceType::LShapedMirror => vec![
                Block::new(spawn_position.clone(), piece_type.clone(), true),
                Block::new(spawn_position.add(&IVec2::new(-1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(-1, 1)), piece_type.clone(), false),
            ],
            PieceType::LShaped => vec![
                Block::new(spawn_position.clone(), piece_type.clone(), true),
                Block::new(spawn_position.add(&IVec2::new(-1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(1, 1)), piece_type.clone(), false),
            ],
            PieceType::Skew => vec![
                Block::new(spawn_position.clone(), piece_type.clone(), true),
                Block::new(spawn_position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(0, 1)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(-1, 1)), piece_type.clone(), false),
            ],
            PieceType::SkewMirror => vec![
                Block::new(spawn_position.clone(), piece_type.clone(), true),
                Block::new(spawn_position.add(&IVec2::new(-1, 0)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(0, 1)), piece_type.clone(), false),
                Block::new(spawn_position.add(&IVec2::new(1, 1)), piece_type.clone(), false),
            ],
        };

        Piece::new(spawn_position, blocks, piece_type)
    }
}
