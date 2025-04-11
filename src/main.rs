use macroquad::prelude::*;
use macroquad::shapes;
use macroquad::{time, window};


pub fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Random Walker".to_owned(),
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(WHITE);
        draw_text("Hello, World!", 20.0, 20.0, 30.0, BLACK);
        draw_rectangle(1.0, 1.0, 20.0, 15.0, BLUE);
        next_frame().await;
    }
}
