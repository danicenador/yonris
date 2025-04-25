use macroquad::prelude;


pub trait InputEngineTrait {
    fn o_key_pressed(&self) -> bool;
    fn i_key_pressed(&self) -> bool;
    fn r_key_pressed(&self) -> bool;
    fn h_key_pressed(&self) -> bool;
    fn j_key_pressed(&self) -> bool;
    fn k_key_pressed(&self) -> bool;
    fn l_key_pressed(&self) -> bool;
    fn space_key_pressed(&self) -> bool;
    fn space_key_pressed_once(&self) -> bool;
    fn enter_key_pressed_once(&self) -> bool;
    fn escape_key_pressed_once(&self) -> bool;
}

pub struct InputEngine;

impl InputEngineTrait for InputEngine {
    fn o_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::O)
    }

    fn i_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::I)
    }

    fn r_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::R)
    }

    fn h_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::H)
    }
    fn j_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::J)
    }
    fn k_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::K)
    }
    fn l_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::L)
    }
    fn space_key_pressed(&self) -> bool {
        prelude::is_key_down(prelude::KeyCode::Space)
    }
    fn space_key_pressed_once(&self) -> bool {
        prelude::is_key_pressed(prelude::KeyCode::Space)
    }
    fn enter_key_pressed_once(&self) -> bool {
        prelude::is_key_pressed(prelude::KeyCode::Enter)
    }
    fn escape_key_pressed_once(&self) -> bool {
        prelude::is_key_pressed(prelude::KeyCode::Escape)
    }
}
