use crate::pala8::color::Color;
use crate::pala8::constants::INITIAL_SCALING;
use crate::pala8::display_engine;
use crate::pala8::draw_engine;
use crate::pala8::vec2::Vec2;

pub struct GraphicEngine {
    scale: f32,
    draw_engine: Box<dyn draw_engine::DrawEngineTrait>,
}

impl GraphicEngine {
    pub fn new() -> Self {
        let drawer: draw_engine::DrawEngine = draw_engine::DrawEngine;
        Self {
            scale: INITIAL_SCALING,
            draw_engine: Box::new(drawer),
        }
    }

    pub fn draw_background(&self, color: &Color) {
        self.draw_engine.background_color(color);
    }

    pub fn draw_pixel(&self, coordinates: &Vec2, color: &Color) {
        let display_coordinates = Vec2::new(
            display_engine::resolution_transform(coordinates.x, &self.scale),
            display_engine::resolution_transform(coordinates.y, &self.scale),
        );

        self.draw_engine
            .draw_rectangle(&display_coordinates, self.scale, self.scale, &color);
    }

    pub fn draw_rectangle(&self, top_left_pixel: &Vec2, height: f32, width: f32, color: &Color) {
        let display_top_left = Vec2::new(
            display_engine::resolution_transform(top_left_pixel.x, &self.scale),
            display_engine::resolution_transform(top_left_pixel.y, &self.scale),
        );

        let display_height: f32 = height * self.scale;
        let display_width: f32 = width * self.scale;

        self.draw_engine
            .draw_rectangle(&display_top_left, display_width, display_height, &color);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}
