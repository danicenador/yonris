use yonris::playfield;
use yonris::ivec2::IVec2;
use yonris::constants::{PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH};


#[test]
fn test_playfield_valid_possition1() {
    let playfield = playfield::Playfield::new();
    let new_possition: IVec2 = IVec2::new(PLAYFIELD_WIDTH - 1, PLAYFIELD_HEIGHT - 1);
    assert_eq!(playfield.is_valid_position(&new_possition), true);
}

#[test]
fn test_playfield_valid_possition2() {
    let playfield = playfield::Playfield::new();
    let new_possition: IVec2 = IVec2::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT - 1);
    assert_eq!(playfield.is_valid_position(&new_possition), true);
}

#[test]
fn test_playfield_invalid_possition1() {
    let playfield = playfield::Playfield::new();
    let new_possition: IVec2 = IVec2::new(0, PLAYFIELD_HEIGHT - 1);
    assert_eq!(playfield.is_valid_position(&new_possition), false);
}

#[test]
fn test_playfield_invalid_possition2() {
    let playfield = playfield::Playfield::new();
    let new_possition: IVec2 = IVec2::new(PLAYFIELD_WIDTH + 3, PLAYFIELD_HEIGHT - 1);
    assert_eq!(playfield.is_valid_position(&new_possition), false);
}
