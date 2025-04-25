use crate::playfield::Playfield;
use crate::input;

pub enum GameState {
    StartScreen,
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
pub struct GameCore {
    game_state: GameState,
    playfield: Playfield,
    gameplay_input: input::GameplayInput,
    menu_input: input::MenuInput,
}
impl GameCore {
    pub fn new() -> Self {
        let playfield = Playfield::new();
        let gameplay_input = input::GameplayInput::new();
        let menu_input = input::MenuInput::new();
        Self {
            game_state: GameState::StartScreen,
            playfield,
            gameplay_input,
            menu_input,
        }
    }

    pub fn run(&mut self, frame_time: f32) -> bool {
        match self.game_state {
            GameState::StartScreen => {
                if self.menu_input.start_game() {
                    self.game_state = GameState::MainMenu;
                }
            }
            GameState::MainMenu => {
                if self.menu_input.start_game() {
                    self.playfield.reset();
                    self.game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                if self.menu_input.pause_game() {
                    self.game_state = GameState::Paused;
                } else if self.playfield.is_game_over() {
                    self.game_state = GameState::GameOver;
                } else {
                    self.gameplay_input.update_fall(&mut self.playfield, frame_time);
                    self.gameplay_input.apply_input(&mut self.playfield, frame_time);
                }
            }
            GameState::Paused => {
                if self.menu_input.start_game() {
                    self.game_state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if self.menu_input.start_game() {
                    self.game_state = GameState::MainMenu;
                }
            }
        }
        true
    }

    pub fn get_playfield(&self) -> &Playfield {
        &self.playfield
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

}
