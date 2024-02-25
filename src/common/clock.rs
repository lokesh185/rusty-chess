use crate::common::game_modes::GameType;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Clock {
    black_time: u32,
    white_time: u32,
}
impl Default for Clock {
    fn default() -> Self {
        Self {
            black_time: 300,
            white_time: 300,
        }
    }
}
impl Clock {
    pub fn from_game_type(game_type: GameType) -> Self {
        Self {
            black_time: game_type.time_control(),
            white_time: game_type.time_control(),
        }
    }
}
