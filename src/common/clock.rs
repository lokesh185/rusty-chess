use crate::common::game_modes::GameType;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::logic::PlayerType;
#[derive(Deserialize, Serialize)]
pub struct Clock {
    pub black_time: u64,
    pub white_time: u64,
    last_update: u64,
}
impl Default for Clock {
    fn default() -> Self {
        Self {
            black_time: 300,
            white_time: 300,
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64,
        }
    }
}
impl Clock {
    pub fn from_game_type(game_type: GameType) -> Self {
        Self {
            black_time: game_type.time_control(),
            white_time: game_type.time_control(),
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64,
        }
    }
    pub fn update_time(&mut self, cur_player: PlayerType) -> Option<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;
        if now > self.last_update {
            let elapsed = now - self.last_update;
            self.last_update = now;
            match cur_player {
                PlayerType::Black => self.black_time = self.black_time.checked_sub(elapsed)?,
                PlayerType::White => self.white_time = self.white_time.checked_sub(elapsed)?,
            };
        };
        Some(())
    }
}
