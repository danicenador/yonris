use crate::constants::{PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH, INITIAL_FALL_PACE};
use crate::piece::Piece;
use crate::ivec2::IVec2;
use crate::block::Block;
use crate::piece_factory::PieceFactory;
use crate::piece::PieceType;
use crate::utils;

const STARTING_X: i32 = 6;
const STARTING_Y: i32 = 1;

pub struct Playfield {
    pub width: i32,
    pub height: i32,
    pub piece: Piece,
    pub stacked_blocks: Vec<Block>,
    fall: FallDefinition,
    piece_factory: PieceFactory,
    spawn_position: IVec2,
    game_over: bool,
}

impl Playfield {
    pub fn new() -> Self {
        let width: i32 = PLAYFIELD_WIDTH;
        let height: i32 = PLAYFIELD_HEIGHT;
        let fall: FallDefinition = FallDefinition {
            pace: INITIAL_FALL_PACE,
            buffer: 0.0,
        };
        let piece_factory: PieceFactory = PieceFactory::new();
        let spawn_position: IVec2 = IVec2::new(STARTING_X, STARTING_Y);
        let stacked_blocks: Vec<Block> = vec![];

        Playfield {
            width,
            height,
            piece: piece_factory.get(spawn_position),
            fall,
            piece_factory,
            spawn_position,
            stacked_blocks,
            game_over: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.fall.buffer += delta_time;
        if self.fall.buffer >= self.fall.pace {
            self.move_piece_down();
            self.fall.buffer = 0.0;
        }
    }

    pub fn rotate_piece(&mut self) {
        if self.piece.piece_type != PieceType::Square {
            let new_positions: Vec<IVec2> = self.piece.projection_rotation();
            let mut valid_positions: Vec<bool> = vec![];

            for possition in &new_positions {
                let is_valid = self.is_valid_position(possition);
                valid_positions.push(is_valid);
            }

            if valid_positions.iter().all(|&x| x) {
                self.piece.set_block_possitions(new_positions);
            }
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
        let move_succesful: bool = self.move_piece(|block| block.projection_down());

        if !move_succesful {
            self.piece_blocks_to_stacked_blocks();
            self.remove_full_rows();
            self.new_piece();
        }
    }

    pub fn is_valid_position(&self, new_possition: &IVec2) -> bool {
        if new_possition.x <= 0 || new_possition.x > self.width {
            return false;
        } else if new_possition.y <= 0 || new_possition.y > self.height {
            return false;
        } else {
            for block in &self.stacked_blocks {
                if block.position == *new_possition {
                    return false;
                }
            }
        }

        true
    }

    fn piece_blocks_to_stacked_blocks(&mut self) {
        for block in &self.piece.blocks {
            self.stacked_blocks.push(block.clone());
        }
    }

    fn get_full_rows(&mut self) -> Vec<i32> {
        let mut blocks_row: Vec<i32> = vec![];
        for block in &self.stacked_blocks {
            blocks_row.push(block.position.y);
        }

        let blocks_per_row = utils::count_occurrences(blocks_row);

        let mut rows_to_remove: Vec<i32> = vec![];
        for (row, count) in blocks_per_row {
            if count >= self.width as usize {
                rows_to_remove.push(row);
            }
        }
        rows_to_remove.sort();
        rows_to_remove
    }

    fn remove_full_rows(&mut self) {
        let rows_to_remove: Vec<i32> = self.get_full_rows();

        self.stacked_blocks.retain(|block| !rows_to_remove.contains(&block.position.y));


        for row in &rows_to_remove {
            for block in &mut self.stacked_blocks {
                if block.position.y < *row {
                    block.set_position(block.projection_down());
                }
            }
        }
    }

    fn new_piece(&mut self) {
        let new_piece: Piece =  self.piece_factory.get(self.spawn_position.clone());

        let mut valid_positions: Vec<bool> = vec![];
        for block in &new_piece.blocks {
            let is_valid = self.is_valid_position(&block.position);
            valid_positions.push(is_valid);
        }

        if valid_positions.iter().all(|&x| x) {
            self.piece = new_piece;
        } else {
            self.game_over = true;
        }

    }

    pub fn reset(&mut self) {
        self.stacked_blocks.clear();
        self.fall.buffer = 0.0;
        self.fall.pace = INITIAL_FALL_PACE;
        self.game_over = false;
        self.new_piece();
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}


struct FallDefinition {
    pub pace: f32,
    pub buffer: f32,
}
