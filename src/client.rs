use crate::common::{clock::Clock, game_modes::GameType, logic::GameState};
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameClient {
    game_state: GameState,
    game_type: Option<GameType>,
    has_started: bool,
    clock: Clock,
}

impl Default for GameClient {
    fn default() -> Self {
        Self {
            game_state: GameState::default(),
            game_type: None,
            has_started: false,
            clock: Clock::default(),
        }
    }
}
impl GameClient {
    pub fn new_local(game_type: GameType) -> Self {
        GameClient {
            game_state: GameState::default(),
            game_type: Some(game_type),
            has_started: true,
            clock: Clock::from_game_type(game_type),
        }
    }
}
