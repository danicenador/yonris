use macroquad::prelude;

use yonris::pala8::color::Color;
use yonris::pala8::display_engine::window_conf;
use yonris::pala8::graphic_engine::GraphicEngine;
use yonris::pala8::vec2::Vec2;

use yonris::drawers::PlayfieldDrawer;
use yonris::input::GameplayInput;
use yonris::playfield::Playfield;
use yonris::game_core::GameCore;

#[macroquad::main(window_conf)]
async fn main() {
    let mut graphic_engine = GraphicEngine::new();

    let red: Color = Color {
        r: 94.0 / 128.0,
        g: 17.0 / 128.0,
        b: 25.0 / 128.0,
        a: 1.0,
    };

    let black: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    let mut core: GameCore = GameCore::new();
    let playfield_drawer: PlayfieldDrawer = PlayfieldDrawer::new(core.get_playfield(), black);
    let mut gameplay_input: GameplayInput = GameplayInput::new();

    loop {
        graphic_engine.draw_background(&red);
        // process input -> update game -> render
        let frame_time: f32 = graphic_engine.get_frame_time();
        let output: bool = core.run(frame_time);
        playfield_drawer.draw(core.get_playfield(), &graphic_engine);
        prelude::next_frame().await
    }
}
