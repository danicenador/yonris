use crate::playfield::Playfield;
use crate::ivec2::IVec2;
use crate::pala8::vec2::Vec2;
use crate::pala8::display_engine::center_line;
use crate::pala8::graphic_engine::GraphicEngine;
use crate::pala8::color::Color;

use crate::constants::PLAYFIELD_BLOCK_PX;
use crate::pala8::constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};


pub struct PlayfieldDrawer {
    pub top_left_pixel: Vec2,
    color: Color,
}

impl PlayfieldDrawer {
    pub fn new(playfield: &Playfield, color: Color) -> Self {
        let pixel_width: i32 = PLAYFIELD_BLOCK_PX * playfield.width;
        let pixel_height: i32 = PLAYFIELD_BLOCK_PX * playfield.height;
        let top_left_pixel: Vec2 = Vec2::new(
            center_line(pixel_width, RESOLUTION_WIDTH),
            center_line(pixel_height, RESOLUTION_HEIGHT),
        );

        PlayfieldDrawer {
            top_left_pixel,
            color,
        }
    }

    pub fn draw(&self, playfield: &Playfield, graphic_engine: &GraphicEngine) {
        self.draw_playfield_background(playfield, graphic_engine);
        self.draw_piece(playfield, graphic_engine);
    }

    fn draw_playfield_background(&self, playfield: &Playfield, graphic_engine: &GraphicEngine){
        graphic_engine.draw_rectangle(
            &self.top_left_pixel,
            (playfield.height * PLAYFIELD_BLOCK_PX) as f32,
            (playfield.width * PLAYFIELD_BLOCK_PX) as f32,
            &self.color,
        )
    }

    fn draw_piece(&self, playfield: &Playfield, graphic_engine: &GraphicEngine) {
        let block_px: i32 = PLAYFIELD_BLOCK_PX;
        let playfield_top_left_pixel: Vec2 = self.top_left_pixel;
        let piece_top_left_pixel: Vec2 = get_piece_top_left_pixel(
            &playfield.piece.position,
            block_px,
            &playfield_top_left_pixel
        );

        graphic_engine.draw_rectangle(
            &piece_top_left_pixel,
            block_px as f32,
            block_px as f32,
            &playfield.piece.color,
        );
    }
}

fn get_piece_top_left_pixel(playfield_coordinates: &IVec2, block_px: i32, playfield_top_left_pixel: &Vec2) -> Vec2 {
    let x: f32 = playfield_top_left_pixel.x + ((playfield_coordinates.x - 1) * block_px) as f32;
    let y: f32 = playfield_top_left_pixel.y + ((playfield_coordinates.y - 1) * block_px) as f32;
    Vec2::new(x, y)
}
