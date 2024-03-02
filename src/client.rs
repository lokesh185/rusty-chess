use crate::common::{
    clock::Clock,
    game_modes::{GameMode, GameType},
    logic::{ChessPosition, GameState},
};
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameClient {
    pub game_state: GameState,
    pub game_type: Option<GameType>,
    has_started: bool,
    pub clock: Clock,
    pub game_mode: Option<GameMode>,
}

impl Default for GameClient {
    fn default() -> Self {
        Self {
            game_state: GameState::default(),
            game_type: None,
            has_started: false,
            clock: Clock::default(),
            game_mode: None,
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
            game_mode: Some(GameMode::Local),
        }
    }
    pub fn update_fen(&mut self, fen: &str) -> Option<()> {
        self.game_state = GameState::from_fen(fen)?;
        Some(())
    }
    pub fn possible_move_ends(&self, pos: &ChessPosition) -> Vec<ChessPosition> {
        self.game_state
            .generate_legal_moves_for_pos(pos)
            .iter()
            .map(|m| m.move_vector.to_pos)
            .collect()
    }
}
