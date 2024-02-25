use serde::{Deserialize, Serialize};
struct GameProperties {
    pub flip_board: bool,
    pub timer: u32,
}

enum GameMode {
    Local(GameProperties),
    Online(GameProperties),
}
impl GameMode {
    pub fn new_local() -> Self {
        GameMode::Local(GameProperties {
            flip_board: false,
            timer: 600,
        })
    }
    pub fn new_online() -> Self {
        GameMode::Online(GameProperties {
            flip_board: false,
            timer: 600,
        })
    }
}
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GameType {
    Blitz,
    Rapid,
    Classical,
    Bullet,
}
impl GameType {
    pub fn time_control(&self) -> u32 {
        match self {
            GameType::Blitz => 300,
            GameType::Rapid => 600,
            GameType::Classical => 1800,
            GameType::Bullet => 60,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            GameType::Blitz => "Blitz".to_string(),
            GameType::Rapid => "Rapid".to_string(),
            GameType::Classical => "Classical".to_string(),
            GameType::Bullet => "Bullet".to_string(),
        }
    }
}
