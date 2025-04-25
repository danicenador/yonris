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
                if self.menu_input.start_game_key() {
                    self.game_state = GameState::MainMenu;
                }
            }
            GameState::MainMenu => {
                if self.menu_input.start_game_key() {
                    self.game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                self.gameplay_input.update_fall(&mut self.playfield, frame_time);
                self.gameplay_input.apply_input(&mut self.playfield, frame_time);
            }
            GameState::Paused => {
            }
            GameState::GameOver => {
            }
        }
        true
    }

    pub fn get_playfield(&self) -> &Playfield {
        &self.playfield
    }

}
