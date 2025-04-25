use std::collections::HashMap;

use crate::pala8::vec2::Vec2;
use crate::pala8::display_engine::center_line;
use crate::pala8::graphic_engine::GraphicEngine;
use crate::pala8::color::Color;
use crate::pala8::sprite::Sprite;
use crate::pala8::letter_sprites::LetterSprites;
use crate::pala8::constants::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};

use crate::constants::PLAYFIELD_BLOCK_PX;
use crate::sprite_loader;
use crate::playfield::Playfield;
use crate::block::Block;
use crate::piece::PieceType;
use crate::ivec2::IVec2;
use crate::game_core::GameCore;
use crate::game_core::GameState;

pub struct Drawer {
    playfield_drawer: PlayfieldDrawer,
    graphic_engine: GraphicEngine,
}
impl Drawer {
    pub fn new(playfield: &Playfield, color: Color) -> Self {
        let playfield_drawer = PlayfieldDrawer::new(playfield, color);
        let graphic_engine = GraphicEngine::new();
        Drawer { playfield_drawer, graphic_engine }
    }

    pub fn draw(&self, game_core: &GameCore) {
        self.draw_background();
        match game_core.get_game_state() {
            GameState::StartScreen => {
                self.playfield_drawer.draw_text("Start Screen", &Vec2::new(10.0, 40.0), &self.graphic_engine);
            }
            GameState::MainMenu => {
                self.playfield_drawer.draw_text("Main Menu", &Vec2::new(20.0, 100.0), &self.graphic_engine);
            }
            GameState::Playing => {
                self.playfield_drawer.draw(game_core.get_playfield(), &self.graphic_engine);
            }
            GameState::Paused => {
                self.playfield_drawer.draw_text("Paused", &Vec2::new(20.0, 100.0), &self.graphic_engine);
            }
            GameState::GameOver => {
                self.playfield_drawer.draw_text("Game Over", &Vec2::new(20.0, 100.0), &self.graphic_engine);
            }
        }
    }

    pub fn get_frame_time(&self) -> f32 {
        self.graphic_engine.get_frame_time()
    }

    fn draw_background(&self) {
        let red: Color = Color {
            r: 94.0 / 128.0,
            g: 17.0 / 128.0,
            b: 25.0 / 128.0,
            a: 1.0,
        };
        self.graphic_engine.draw_background(&red);
    }

}


pub struct PlayfieldDrawer {
    pub top_left_pixel: Vec2,
    color: Color,
    piece_sprites: HashMap<PieceType, Sprite>,
    letter_sprites: LetterSprites,
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
        let letter_sprites: LetterSprites = LetterSprites::new();

        PlayfieldDrawer {
            top_left_pixel,
            color,
            piece_sprites,
            letter_sprites,
        }
    }

    pub fn draw(&self, playfield: &Playfield, graphic_engine: &GraphicEngine) {
        self.draw_playfield_background(playfield, graphic_engine);
        for block in &playfield.piece.blocks {
            self.draw_block(block, graphic_engine);
        }
        for block in &playfield.stacked_blocks {
            self.draw_block(block, graphic_engine);
        }
    }

    fn draw_playfield_background(&self, playfield: &Playfield, graphic_engine: &GraphicEngine){
        graphic_engine.draw_rectangle(
            &self.top_left_pixel,
            (playfield.height * PLAYFIELD_BLOCK_PX) as f32,
            (playfield.width * PLAYFIELD_BLOCK_PX) as f32,
            &self.color,
        )
    }

    fn draw_block(&self, block: &Block, graphic_engine: &GraphicEngine) {
        let piece_top_left_pixel = self.calculate_block_top_left_pixel(&block);
        let piece_type = &block.piece_type;

        match self.piece_sprites.get(piece_type) {
            Some(sprite) => {
                self.draw_sprite(graphic_engine, sprite, &piece_top_left_pixel);
            }
            None => {
                eprintln!("Warning: No sprite found for piece type {:?}", piece_type);
            }
        }
    }

    fn calculate_block_top_left_pixel(&self, block: &Block) -> Vec2 {
        let block_px: i32 = PLAYFIELD_BLOCK_PX;
        get_piece_top_left_pixel(
            &block.position,
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

    pub fn draw_text(&self, text: &str, top_left_pixel: &Vec2, graphic_engine: &GraphicEngine) {
        let mut x_offset = 0.0;
        for letter in text.chars() {
            if let Some(sprite) = self.letter_sprites.get_char(letter) {
                let letter_top_left_pixel = Vec2::new(top_left_pixel.x + x_offset, top_left_pixel.y);
                self.draw_sprite(graphic_engine, sprite, &letter_top_left_pixel);
            }
            x_offset += self.letter_sprites.get_letter_width() as f32;
        }
    }
}

fn get_piece_top_left_pixel(playfield_coordinates: &IVec2, block_px: i32, playfield_top_left_pixel: &Vec2) -> Vec2 {
    let x: f32 = playfield_top_left_pixel.x + ((playfield_coordinates.x - 1) * block_px) as f32;
    let y: f32 = playfield_top_left_pixel.y + ((playfield_coordinates.y - 1) * block_px) as f32;
    Vec2::new(x, y)
}
