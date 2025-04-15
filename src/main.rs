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

    let color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    let blue: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

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

    let coordinates: Vec2 = Vec2::new(65.0, 2.0);
    let coordinates2: Vec2 = Vec2::new(65.0, 3.0);
    let coordinates3: Vec2 = Vec2::new(65.0, 4.0);
    let coordinates4: Vec2 = Vec2::new(65.0, 1.0);

    let coordinates5: Vec2 = Vec2::new(65.0, 125.0);
    let coordinates6: Vec2 = Vec2::new(65.0, 126.0);
    let coordinates7: Vec2 = Vec2::new(65.0, 127.0);
    let coordinates8: Vec2 = Vec2::new(65.0, 128.0);

    let playfield: Playfield = Playfield::new();
    let playfield_drawer: PlayfieldDrawer = PlayfieldDrawer::new(&playfield);

    loop {
        graphic_engine.draw_background(&red);
        graphic_engine.draw_rectangle(
            &playfield_drawer.top_left_pixel,
            (playfield.height * PLAYFIELD_BLOCK_PX) as f32,
            (playfield.width * PLAYFIELD_BLOCK_PX) as f32,
            &black,
        );
        graphic_engine.draw_pixel(&coordinates, &color);
        graphic_engine.draw_pixel(&coordinates2, &blue);
        graphic_engine.draw_pixel(&coordinates3, &color);
        graphic_engine.draw_pixel(&coordinates4, &blue);
        graphic_engine.draw_pixel(&coordinates5, &color);
        graphic_engine.draw_pixel(&coordinates6, &blue);
        graphic_engine.draw_pixel(&coordinates7, &color);
        graphic_engine.draw_pixel(&coordinates8, &blue);
        prelude::next_frame().await
    }
}
