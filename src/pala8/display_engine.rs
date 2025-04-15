use crate::pala8::constants;
use macroquad::window;

pub fn window_conf() -> window::Conf {
    let window_heigth: i32 = constants::RESOLUTION_HEIGHT * constants::INITIAL_SCALING as i32;
    let window_width: i32 = constants::RESOLUTION_WIDTH * constants::INITIAL_SCALING as i32;

    window::Conf {
        window_title: "Random Walker".to_owned(),
        window_width: window_width,
        window_height: window_heigth,
        ..Default::default()
    }
}

pub fn resolution_transform(coordinate: f32, scale: &f32) -> f32 {
    (coordinate - 1.0) * scale + 1.0
}

pub fn center_line(longitude: i32, available_distance: i32) -> f32 {
    let starting_point = (available_distance as f32 / 2.0) - (longitude as f32 / 2.0);
    starting_point + 1.0
}
