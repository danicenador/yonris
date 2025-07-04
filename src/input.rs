use crate::pala8::input_engine;

use crate::constants::{PIECE_PACE, PIECE_PACE_FAST};
use crate::playfield::Playfield;

pub struct GameplayInput{
    input_engine: Box<dyn input_engine::InputEngineTrait>,
    left_movement: MovementInput,
    right_movement: MovementInput,
    down_movement: MovementInput,
}

impl GameplayInput {
    pub fn new() -> Self {
        let input_engine: input_engine::InputEngine = input_engine::InputEngine;
        Self {
            input_engine: Box::new(input_engine),
            left_movement: MovementInput::new(),
            right_movement: MovementInput::new(),
            down_movement: MovementInput::new(),
        }
    }

    pub fn update_fall(&mut self, playfield: &mut Playfield, frame_time: f32) {
        if !self.input_engine.j_key_pressed(){
            playfield.update(frame_time);
        }
    }

    pub fn apply_input(&mut self, playfield: &mut Playfield, frame_time: f32) {
        if self.input_engine.h_key_pressed() {
            self.left_movement_logic(playfield, frame_time);} else {self.left_movement.reset();}
        if self.input_engine.l_key_pressed() {
            self.right_movement_logic(playfield, frame_time);} else {self.right_movement.reset();}
        if self.input_engine.j_key_pressed() {
            self.down_movement_logic(playfield, frame_time);} else {self.down_movement.reset();}


        if self.input_engine.space_key_pressed_once() {
            playfield.rotate_piece();
        }
    }

    fn left_movement_logic(&mut self, playfield: &mut Playfield, frame_time: f32)  {
        if !self.left_movement.last_status {
            playfield.move_piece_left();
            self.left_movement.last_status = true;
        } else {
            self.left_movement.time_pressed += frame_time;
            if self.left_movement.time_pressed > PIECE_PACE {
                self.left_movement.time_pressed = 0.0;
                playfield.move_piece_left();
            }
        }
    }

    fn right_movement_logic(&mut self, playfield: &mut Playfield, frame_time: f32)  {
        if !self.right_movement.last_status {
            playfield.move_piece_right();
            self.right_movement.last_status = true;
        } else {
            self.right_movement.time_pressed += frame_time;
            if self.right_movement.time_pressed > PIECE_PACE {
                self.right_movement.time_pressed = 0.0;
                playfield.move_piece_right();
            }
        }
    }

    fn down_movement_logic(&mut self, playfield: &mut Playfield, frame_time: f32)  {
        if !self.down_movement.last_status {
            playfield.move_piece_down();
            self.down_movement.last_status = true;
        } else {
            self.down_movement.time_pressed += frame_time;
            if self.down_movement.time_pressed > PIECE_PACE_FAST {
                self.down_movement.time_pressed = 0.0;
                playfield.move_piece_down();
            }
        }
    }


}


struct MovementInput {
    pub last_status: bool,
    pub time_pressed: f32,
}
impl MovementInput {
    fn new() -> Self {
        let last_status: bool = false;
        let time_pressed: f32 = 0.0;
        Self {
            last_status,
            time_pressed,
        }
    }

    pub fn reset(&mut self) {
        if self.last_status {
            self.last_status = false;
            self.time_pressed = 0.0;
        }
    }
}


pub struct MenuInput {
    input_engine: Box<dyn input_engine::InputEngineTrait>,
}

impl MenuInput {
    pub fn new() -> Self {
        let input_engine: input_engine::InputEngine = input_engine::InputEngine;
        Self {
            input_engine: Box::new(input_engine),
        }
    }

    pub fn start_game(&self) -> bool {
        self.input_engine.enter_key_pressed_once()
    }

    pub fn pause_game(&self) -> bool {
        self.input_engine.escape_key_pressed_once()
    }
}
