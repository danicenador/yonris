use crate::constants::{PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH, INITIAL_FALL_PACE};
use crate::piece::Piece;
use crate::ivec2::IVec2;
use crate::block::Block;
use crate::piece::PieceType;

const STARTING_X: i32 = 6;
const STARTING_Y: i32 = 1;

pub struct Playfield {
    pub width: i32,
    pub height: i32,
    pub piece: Piece,
    pub fall: FallDefinition,
}

impl Playfield {
    pub fn new() -> Self {
        let width: i32 = PLAYFIELD_WIDTH;
        let height: i32 = PLAYFIELD_HEIGHT;
        let fall: FallDefinition = FallDefinition {
            pace: INITIAL_FALL_PACE,
            buffer: 0.0,
        };

        // hardoced piece
        let position: IVec2 = IVec2::new(STARTING_X, STARTING_Y);
        let piece_type = PieceType::LShapedMirror;
        let blocks: Vec<Block> = vec![
            Block::new(position.clone(), piece_type.clone(), true),
            Block::new(position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
            Block::new(position.add(&IVec2::new(-1, 0)), piece_type.clone(), false),
            Block::new(position.add(&IVec2::new(1, 1)), piece_type.clone(), false),

        ];
        let piece: Piece = Piece::new(position, blocks, piece_type);

        Playfield {
            width,
            height,
            piece,
            fall,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.fall.buffer += delta_time;
        if self.fall.buffer >= self.fall.pace {
            self.move_piece_down();
            self.fall.buffer = 0.0;
        }
    }

    fn move_piece<F>(&mut self, projection_fn: F) -> bool
    where
        F: Fn(&Block) -> IVec2,
    {
        /*
        Move the piece in the direction specified by the projection function.
        Args:
            projection_fn: A function that takes a block and returns its new position.
        Returns:
            bool: True if the piece was moved successfully, false otherwise.
        */
        let mut new_positions: Vec<IVec2> = vec![];
        let mut valid_positions: Vec<bool> = vec![];

        for block in &self.piece.blocks {
            let new_position: IVec2 = projection_fn(block);
            let is_valid = self.is_valid_position(&new_position);
            valid_positions.push(is_valid);
            new_positions.push(new_position);
        }

        if valid_positions.iter().all(|&x| x) {
            for (block, new_position) in self.piece.blocks.iter_mut().zip(new_positions) {
                block.set_position(new_position);
            }
            return true;
        }
        false
    }

    pub fn move_piece_left(&mut self) {
        self.move_piece(|block| block.projection_left());
    }

    pub fn move_piece_right(&mut self) {
        self.move_piece(|block| block.projection_right());
    }

    pub fn move_piece_down(&mut self) {
        let output: bool = self.move_piece(|block| block.projection_down());

        if !output {
            let position: IVec2 = IVec2::new(STARTING_X, STARTING_Y);
            let piece_type = PieceType::LShapedMirror;
            let blocks: Vec<Block> = vec![
                Block::new(position.clone(), piece_type.clone(), true),
                Block::new(position.add(&IVec2::new(1, 0)), piece_type.clone(), false),
                Block::new(position.add(&IVec2::new(-1, 0)), piece_type.clone(), false),
                Block::new(position.add(&IVec2::new(1, 1)), piece_type.clone(), false),

            ];
            self.piece = Piece::new(position, blocks, piece_type);
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


struct FallDefinition {
    pub pace: f32,
    pub buffer: f32,
}
