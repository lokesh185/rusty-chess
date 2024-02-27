use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GameMode {
    Local,
    Online,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GameType {
    Blitz,
    Rapid,
    Classical,
    Bullet,
}
impl GameType {
    pub fn time_control(&self) -> u64 {
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
