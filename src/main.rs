use macroquad::prelude;

use yonris::pala8::color::Color;
use yonris::pala8::display_engine::window_conf;

use yonris::drawers::Drawer;
use yonris::game_core::GameCore;

#[macroquad::main(window_conf)]
async fn main() {

    let black: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    let mut core: GameCore = GameCore::new();
    let drawer: Drawer = Drawer::new(core.get_playfield(), black);

    loop {
        // process input -> update game -> render
        let frame_time: f32 = drawer.get_frame_time();
        let output: bool = core.run(frame_time);
        drawer.draw(&core);
        prelude::next_frame().await
    }
}
