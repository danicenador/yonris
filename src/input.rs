use crate::pala8::input_engine;

use crate::playfield::Playfield;

pub struct GameplayInput{
    input_engine: Box<dyn input_engine::InputEngineTrait>,
}

impl GameplayInput {
    pub fn new() -> Self {
        let input_engine: input_engine::InputEngine = input_engine::InputEngine;
        Self {
            input_engine: Box::new(input_engine),
        }
    }

    pub fn apply_input(&self, playfield: &mut Playfield){
        if self.input_engine.h_key_pressed() {
            playfield.piece.move_left();
        }
        if self.input_engine.j_key_pressed() {
            playfield.piece.move_down();
        }
        // if self.input_engine.k_key_pressed() {
        //     playfield.piece.move_up();
        // }
        if self.input_engine.l_key_pressed() {
            playfield.piece.move_right();
        }
    }

}
