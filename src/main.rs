use macroquad::prelude;

use yonris::pala8::color::Color;
use yonris::pala8::display_engine::window_conf;
use yonris::pala8::graphic_engine::GraphicEngine;
use yonris::pala8::vec2::Vec2;

use yonris::drawers::PlayfieldDrawer;
use yonris::playfield::Playfield;

use yonris::constants::PLAYFIELD_BLOCK_PX;

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

    let playfield: Playfield = Playfield::new();
    let playfield_drawer: PlayfieldDrawer = PlayfieldDrawer::new(&playfield, black);


    loop {
        graphic_engine.draw_background(&red);
        playfield_drawer.draw(&graphic_engine);
        prelude::next_frame().await
    }
}
