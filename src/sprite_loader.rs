use crate::pala8::sprite::Sprite;
use crate::constants::{SPRITE_PATH, PLAYFIELD_BLOCK_PX};
use crate::piece::PieceType;
use std::collections::HashMap;

pub fn get_piece_sprites() -> HashMap<PieceType, Sprite> {
    let sprite_sheet: Sprite = Sprite::load(SPRITE_PATH);
    let mut sprite_map: HashMap<PieceType, Sprite> = HashMap::new();
    let sprite_sheet_height: usize = sprite_sheet.get_height();
    let horizontal_step: usize = PLAYFIELD_BLOCK_PX as usize;

    let piece_types = [
        PieceType::Straigth,
        PieceType::Square,
        PieceType::TShaped,
        PieceType::LShapedMirror,
        PieceType::LShaped,
        PieceType::Skew,
        PieceType::SkewMirror,
    ];

    for (i, piece_type) in piece_types.iter().enumerate() {
        let x = i * horizontal_step;
        let sprite = sprite_sheet.slice(x, 0, horizontal_step, sprite_sheet_height);
        sprite_map.insert(piece_type.clone(), sprite);
    }

    sprite_map
}
