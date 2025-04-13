use macroquad::prelude;


pub trait InputEngineTrait {
    fn o_key_pressed(&self) -> bool;
    fn i_key_pressed(&self) -> bool;
    fn r_key_pressed(&self) -> bool;
}

pub struct InputEngine;

impl InputEngineTrait for InputEngine {
    fn o_key_pressed(&self) -> bool {
        prelude::is_key_pressed(prelude::KeyCode::O)
    }

    fn i_key_pressed(&self) -> bool {
        prelude::is_key_pressed(prelude::KeyCode::I)
    }

    fn r_key_pressed(&self) -> bool {
        prelude::is_key_pressed(prelude::KeyCode::R)
    }
}
