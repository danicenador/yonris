use crate::pala8::sprite::Sprite;
use crate::pala8::constants::LETTER_SPRITE_PATH;
use std::collections::HashMap;

const LETTER_WIDTH: usize = 6;
const SUPPORTED_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.!";

pub struct LetterSprites {
    sprite_hash: HashMap<char, Sprite>,
}

impl LetterSprites {
    pub fn new() -> Self {
        let sprite_sheet: Sprite = Sprite::load(LETTER_SPRITE_PATH);
        let sprite_sheet_height: usize = sprite_sheet.get_height();
        let horizontal_step: usize = LETTER_WIDTH;

        let mut sprite_hash: HashMap<char, Sprite> = HashMap::new();
        for (i, letter) in SUPPORTED_LETTERS.chars().enumerate() {
            let x = i * horizontal_step;
            let sprite = sprite_sheet.slice(x, 0, horizontal_step, sprite_sheet_height);
            sprite_hash.insert(letter, sprite);
        }

        LetterSprites { sprite_hash }
    }

    pub fn get_char(&self, letter: char) -> Option<&Sprite> {
        self.sprite_hash.get(&letter.to_ascii_uppercase())
    }

    pub fn get_string(&self, text: &str) -> Vec<Option<&Sprite>> {
        text.chars()
            .map(|c| self.get_char(c))
            .collect()
    }
}
