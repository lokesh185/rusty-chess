use super::logic::{ChessPosition, Piece, PieceType, PlayerType};

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
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChessMove {
    pub move_kind: ChessMoveKind,
    pub move_vector: ChessMoveVector,
    pub moved_piece: Piece,
}

impl ChessMove {
    pub fn new(move_kind: ChessMoveKind, move_vector: ChessMoveVector, moved_piece: Piece) -> Self {
        Self {
            move_kind,
            move_vector,
            moved_piece,
        }
    }
}
impl ToString for ChessMove {
    fn to_string(&self) -> String {
        "todo".to_string()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChessMoveKind {
    Castle(CastleType),
    Promotion(PieceType),
    Normal,
    /// piece type of the taken piece
    Take(PieceType),
    /// postion of the taken pawn
    EnPassant(ChessPosition),
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CastleType {
    Short,
    Long,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveHistory {}
