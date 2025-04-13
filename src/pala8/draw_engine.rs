use macroquad::{prelude, time};

use crate::pala8::color::Color;
use crate::pala8::vec2::Vec2;

pub trait DrawEngineTrait {
    fn draw_rectangle(&self, top_left_coodinates: &Vec2, width: f32, height: f32, color: &Color);
    fn background_color(&self, color: &Color);
    fn get_frame_time(&self) -> f32;
}

pub struct DrawEngine;

impl DrawEngineTrait for DrawEngine {
    fn draw_rectangle(&self, top_left_coodinates: &Vec2, width: f32, height: f32, color: &Color) {
        prelude::draw_rectangle(
            top_left_coodinates.x,
            top_left_coodinates.y,
            width,
            height,
            prelude::Color::new(color.r, color.g, color.b, color.a),
        );
    }

    fn background_color(&self, color: &Color) {
        prelude::clear_background(prelude::Color::new(color.r, color.g, color.b, color.a));
    }

    fn get_frame_time(&self) -> f32 {
        time::get_frame_time()
    }
}
