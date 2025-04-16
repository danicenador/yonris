use yonris::pala8::display_engine;
use yonris::pala8::sprite::Sprite;

#[test]
fn test_resolution_transform() {
    let coordinate: f32 = 5.0;
    let scale: f32 = 5.0;
    let output_resolution = display_engine::resolution_transform(coordinate, &scale);
    assert_eq!(output_resolution, 21.0);
}

#[test]
fn test_center_line() {
    let longitude: i32 = 10;
    let available_distance: i32 = 128;
    let starting_point: f32 = display_engine::center_line(longitude, available_distance);
    assert_eq!(starting_point, 60.0);
}

#[test]
fn test_center_line2() {
    let longitude: i32 = 20*6;
    let available_distance: i32 = 128;
    let starting_point: f32 = display_engine::center_line(longitude, available_distance);
    assert_eq!(starting_point, 5.0);
}


#[test]
fn test_sprite_load(){
    let sprite_path: &str = "assets/sprite.png";
    let sprite: Sprite = Sprite::load(sprite_path);
    assert_eq!(sprite.get_width(), 42);
    assert_eq!(sprite.get_height(), 6)
}

#[test]
fn test_sprite_slice(){
    let sprite_path: &str = "assets/sprite.png";
    let sprite: Sprite = Sprite::load(sprite_path);
    let start_x: usize = 0;
    let start_y: usize = 0;
    let slice_width: usize = 6;
    let slice_height: usize = sprite.get_height();
    let sliced_sprite: Sprite = sprite.slice(start_x, start_y, slice_width, slice_height);

    assert_eq!(sliced_sprite.get_width(), 6);
    assert_eq!(sliced_sprite.get_height(), 6)
}
