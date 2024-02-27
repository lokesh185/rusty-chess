use super::logic::{ChessPosition, Piece, PlayerType};

use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChessMoveVector {
    pub from_pos: ChessPosition,
    pub to_pos: ChessPosition,
}
impl ChessMoveVector {
    pub fn new(from_pos: ChessPosition, to_pos: ChessPosition) -> Self {
        Self { from_pos, to_pos }
    }
}
impl ToString for ChessMoveVector {
    fn to_string(&self) -> String {
        "todo".to_string()
    }
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum ChessMove {
    Castle(CastleType),
    Promotion(Promotion),
    Normal,
    EnPassant(ChessPosition),
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum CastleType {
    Short(PlayerType),
    Long(PlayerType),
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Promotion {
    move_vector: ChessMoveVector,
    promoted_to: Piece,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct NormalMove {
    piece: Piece,
    move_vector: ChessMoveVector,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct EnPassant {
    move_vector: ChessMoveVector,
    taken_pawn_position: ChessPosition,
}

pub struct MoveHistory {}
