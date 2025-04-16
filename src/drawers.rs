use std::collections::HashMap;

use crate::piece::PieceType;
use crate::playfield::Playfield;
use crate::ivec2::IVec2;
use crate::pala8::vec2::Vec2;
use crate::pala8::display_engine::center_line;
use crate::pala8::graphic_engine::GraphicEngine;
use crate::pala8::color::Color;
use crate::pala8::sprite::Sprite;

use crate::constants::PLAYFIELD_BLOCK_PX;
use crate::pala8::constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use crate::sprite_loader;

pub struct PlayfieldDrawer {
    pub top_left_pixel: Vec2,
    color: Color,
    piece_sprites: HashMap<PieceType, Sprite>,
}

impl PlayfieldDrawer {
    pub fn new(playfield: &Playfield, color: Color) -> Self {
        let pixel_width: i32 = PLAYFIELD_BLOCK_PX * playfield.width;
        let pixel_height: i32 = PLAYFIELD_BLOCK_PX * playfield.height;
        let top_left_pixel: Vec2 = Vec2::new(
            center_line(pixel_width, RESOLUTION_WIDTH),
            center_line(pixel_height, RESOLUTION_HEIGHT),
        );
        let piece_sprites: HashMap<PieceType, Sprite> = sprite_loader::get_piece_sprites();

        PlayfieldDrawer {
            top_left_pixel,
            color,
            piece_sprites
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
        let piece_top_left_pixel = self.calculate_piece_top_left_pixel(playfield);
        let piece_type = &playfield.piece.piece_type;

        match self.piece_sprites.get(piece_type) {
            Some(sprite) => {
                self.draw_sprite(graphic_engine, sprite, &piece_top_left_pixel);
            }
            None => {
                eprintln!("Warning: No sprite found for piece type {:?}", piece_type);
            }
        }
    }

    fn calculate_piece_top_left_pixel(&self, playfield: &Playfield) -> Vec2 {
        let block_px: i32 = PLAYFIELD_BLOCK_PX;
        get_piece_top_left_pixel(
            &playfield.piece.position,
            block_px,
            &self.top_left_pixel,
        )
    }

    fn draw_sprite(&self, graphic_engine: &GraphicEngine, sprite: &Sprite, top_left_pixel: &Vec2) {
        let sprite_height = sprite.get_height();

        for row in 0..sprite.get_width() {
            for col in 0..sprite_height {
                graphic_engine.draw_pixel(
                    &Vec2::new(
                        top_left_pixel.x + col as f32,
                        top_left_pixel.y + row as f32,
                    ),
                    &sprite.get_pixel(col, row),
                );
            }
        }
    }
}

fn get_piece_top_left_pixel(playfield_coordinates: &IVec2, block_px: i32, playfield_top_left_pixel: &Vec2) -> Vec2 {
    let x: f32 = playfield_top_left_pixel.x + ((playfield_coordinates.x - 1) * block_px) as f32;
    let y: f32 = playfield_top_left_pixel.y + ((playfield_coordinates.y - 1) * block_px) as f32;
    Vec2::new(x, y)
}
