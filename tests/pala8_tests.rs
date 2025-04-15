use yonris::pala8::display_engine; // Adjust this path based on how `color` is exported in `mod.rs`

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
