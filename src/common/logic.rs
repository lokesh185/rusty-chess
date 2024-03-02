use serde::{Deserialize, Serialize};

use crate::common::fen::GameData;

use super::move_history::{CastleType, ChessMove, ChessMoveKind, ChessMoveVector};
#[derive(Debug, Clone, Copy)]
pub enum GameEnd {
    Checkmate,
    StaleMate,
    Draw,
}
impl ToString for GameEnd {
    fn to_string(&self) -> String {
        match self {
            Self::Checkmate => "Checkmate".to_string(),
            Self::StaleMate => "Stalemate".to_string(),
            Self::Draw => "Draw".to_string(),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum PlayerType {
    Black,
    White,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Piece {
    pub piece_kind: PieceType,
    pub player_kind: PlayerType,
}

impl Piece {
    pub fn new(piece_kind: PieceType, player_kind: PlayerType) -> Piece {
        Self {
            piece_kind: piece_kind,
            player_kind: player_kind,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChessPosition {
    pub file: i32,
    pub rank: i32,
}

fn check_valid_pos(pos: ChessPosition) -> Option<ChessPosition> {
    if pos.file >= 0 && pos.file < 8 && pos.rank >= 0 && pos.rank < 8 {
        Some(pos)
    } else {
        None
    }
}
impl ChessPosition {
    pub fn new(file: i32, rank: i32) -> Option<ChessPosition> {
        check_valid_pos(Self { file, rank })
    }
    pub fn from_tuple((file, rank): (usize, usize)) -> Option<ChessPosition> {
        check_valid_pos(Self {
            file: file as i32,
            rank: rank as i32,
        })
    }
    pub fn add(&self, other: &ChessPosition) -> Option<ChessPosition> {
        check_valid_pos(Self {
            file: self.file + other.file,
            rank: self.rank + other.rank,
        })
    }
    //add offset to current position
    pub fn add_offset(&self, offset: &(i32, i32)) -> Option<ChessPosition> {
        check_valid_pos(Self {
            file: self.file + offset.0,
            rank: self.rank + offset.1,
        })
    }
    pub fn mul(&self, other: i32) -> Option<ChessPosition> {
        check_valid_pos(Self {
            file: self.file * other,
            rank: self.rank * other,
        })
    }
    pub fn abs_rank_diff(&self, other: &ChessPosition) -> i32 {
        (self.rank - other.rank).abs()
    }
    pub fn abs_file_diff(&self, other: &ChessPosition) -> i32 {
        (self.file - other.file).abs()
    }
    pub fn as_tuple(&self) -> (i32, i32) {
        (self.file, self.rank)
    }
    pub fn iter() -> impl Iterator<Item = ChessPosition> {
        (0..8).flat_map(move |file| (0..8).map(move |rank| ChessPosition { file, rank }))
    }
    pub fn to_flipped(self) -> Self {
        Self {
            file: self.file,
            rank: 7 - self.rank,
        }
    }
    pub fn adjust_for_current_player(self, player_kind: PlayerType) -> Self {
        match player_kind {
            PlayerType::Black => self,
            PlayerType::White => self.to_flipped(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChessBoard(pub [[Option<Piece>; 8]; 8]);
impl ChessBoard {
    pub fn get_piece_at_pos(&self, pos: &ChessPosition) -> Option<Piece> {
        self.0[pos.rank as usize][pos.file as usize]
    }
    fn set_piece_at_pos(&mut self, pos: &ChessPosition, piece: Piece) {
        self.0[pos.rank as usize][pos.file as usize] = Some(piece);
    }
    fn remove_piece_at_pos(&mut self, pos: &ChessPosition) {
        self.0[pos.rank as usize][pos.file as usize] = None;
    }
    fn perform_move(&mut self, chess_move: &ChessMove) {
        match chess_move.move_kind {
            ChessMoveKind::Normal | ChessMoveKind::Take(_) => {
                self.set_piece_at_pos(&chess_move.move_vector.to_pos, chess_move.moved_piece);
                self.remove_piece_at_pos(&chess_move.move_vector.from_pos);
            }
            ChessMoveKind::EnPassant(en_pass_pos) => {
                self.set_piece_at_pos(&chess_move.move_vector.to_pos, chess_move.moved_piece);
                self.remove_piece_at_pos(&chess_move.move_vector.from_pos);
                self.remove_piece_at_pos(&en_pass_pos);
            }
            ChessMoveKind::Castle(castle_type) => match castle_type {
                CastleType::Short => {
                    self.set_piece_at_pos(
                        &ChessPosition::new(6, chess_move.move_vector.from_pos.rank).unwrap(),
                        chess_move.moved_piece,
                    );
                    self.remove_piece_at_pos(&chess_move.move_vector.from_pos);
                    self.set_piece_at_pos(
                        &ChessPosition::new(5, chess_move.move_vector.from_pos.rank).unwrap(),
                        self.get_piece_at_pos(
                            &ChessPosition::new(7, chess_move.move_vector.from_pos.rank).unwrap(),
                        )
                        .unwrap(),
                    );
                    self.remove_piece_at_pos(
                        &ChessPosition::new(7, chess_move.move_vector.from_pos.rank).unwrap(),
                    );
                }
                CastleType::Long => {
                    self.set_piece_at_pos(
                        &ChessPosition::new(2, chess_move.move_vector.from_pos.rank).unwrap(),
                        chess_move.moved_piece,
                    );
                    self.remove_piece_at_pos(&chess_move.move_vector.from_pos);
                    self.set_piece_at_pos(
                        &ChessPosition::new(3, chess_move.move_vector.from_pos.rank).unwrap(),
                        self.get_piece_at_pos(
                            &ChessPosition::new(0, chess_move.move_vector.from_pos.rank).unwrap(),
                        )
                        .unwrap(),
                    );
                    self.remove_piece_at_pos(
                        &ChessPosition::new(0, chess_move.move_vector.from_pos.rank).unwrap(),
                    );
                }
            },
            ChessMoveKind::Promotion(piece_type) => {
                self.set_piece_at_pos(
                    &chess_move.move_vector.to_pos,
                    Piece::new(piece_type, chess_move.moved_piece.player_kind),
                );
                self.remove_piece_at_pos(&chess_move.move_vector.from_pos);
            }
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub board: ChessBoard,
    pub active_color: PlayerType,
    /// white short , white long , black short , black long
    castling_availability: (bool, bool, bool, bool),
    en_passant_target_square: Option<ChessPosition>,
    pub half_move_clock: u32,
    pub full_move_no: u32,

    pub prev_moves: Vec<ChessMove>,
    pub promotion: bool,

    pub recurrance: bool,
}

impl Default for GameState {
    fn default() -> Self {
        let def =
            GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let en_pass = if let Some(ele) = def.en_passant_target_square {
            ChessPosition::from_tuple((ele.0, ele.1))
        } else {
            None
        };

        Self {
            board: ChessBoard(def.table),
            active_color: def.active_color,
            castling_availability: def.castling_availability,
            en_passant_target_square: en_pass,
            half_move_clock: def.halfmove_clock,
            full_move_no: def.fullmove_number,
            prev_moves: vec![],
            promotion: false,
            recurrance: false,
        }
    }
}

impl GameState {
    pub fn from_fen(fen_string: &str) -> Option<Self> {
        let def = GameData::from_fen(fen_string).unwrap();

        let en_pass = if let Some(ele) = def.en_passant_target_square {
            ChessPosition::from_tuple((ele.0, ele.1))
        } else {
            None
        };

        Some(Self {
            board: ChessBoard(def.table),
            active_color: def.active_color,
            castling_availability: def.castling_availability,
            en_passant_target_square: en_pass,
            half_move_clock: def.halfmove_clock,
            full_move_no: def.fullmove_number,
            prev_moves: vec![],
            promotion: false,
            recurrance: false,
        })
    }
    fn generate_pseudo_legal_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let pos = ChessPosition::new(file, rank).unwrap();
                // if current piece is of active color then only generate moves
                if self
                    .board
                    .get_piece_at_pos(&pos)
                    .is_some_and(|piece| piece.player_kind == self.active_color)
                {
                    moves.append(&mut self.generate_pseudo_legal_moves_for_pos(&pos));
                }
            }
        }
        moves
    }
    fn generate_legal_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let pos = ChessPosition::new(file, rank).unwrap();
                // if current piece is of active color then only generate moves
                if self
                    .board
                    .get_piece_at_pos(&pos)
                    .is_some_and(|piece| piece.player_kind == self.active_color)
                {
                    moves.append(&mut self.generate_legal_moves_for_pos(&pos));
                }
            }
        }
        moves
    }
    pub fn generate_pseudo_legal_moves_for_pos(&self, pos: &ChessPosition) -> Vec<ChessMove> {
        let piece_option = self.board.get_piece_at_pos(pos);
        if piece_option.is_some_and(|f| f.player_kind != self.active_color) {
            return vec![];
        }
        match piece_option {
            Some(piece) => match (piece.piece_kind, piece.player_kind) {
                (PieceType::Pawn, PlayerType::Black) => {
                    let mut moves = vec![];
                    if let Some(final_pos) = ChessPosition::new(pos.file, pos.rank - 1) {
                        if self.board.get_piece_at_pos(&final_pos).is_none() {
                            moves.push(ChessMove::new(
                                ChessMoveKind::Normal,
                                ChessMoveVector::new(*pos, final_pos),
                                piece,
                            ));
                            if let Some(final_pos) = ChessPosition::new(pos.file, pos.rank - 2) {
                                if self.board.get_piece_at_pos(&final_pos).is_none()
                                    && pos.rank == 6
                                {
                                    moves.push(ChessMove::new(
                                        ChessMoveKind::Normal,
                                        ChessMoveVector::new(*pos, final_pos),
                                        piece,
                                    ));
                                }
                            }
                        }
                    }

                    [(-1, -1), (1, -1)]
                        .iter()
                        .filter_map(|(file, rank)| {
                            Some(ChessPosition::new(pos.file + file, pos.rank + rank)?)
                        })
                        .filter_map(|final_pos| {
                            if self
                                .board
                                .get_piece_at_pos(&final_pos)
                                .is_some_and(|x| x.player_kind != self.active_color)
                            {
                                Some(ChessMove::new(
                                    ChessMoveKind::Take(piece.piece_kind),
                                    ChessMoveVector::new(*pos, final_pos),
                                    piece,
                                ))
                            } else if final_pos == self.en_passant_target_square? {
                                Some(ChessMove::new(
                                    ChessMoveKind::EnPassant(self.en_passant_target_square?),
                                    ChessMoveVector::new(*pos, final_pos),
                                    piece,
                                ))
                            } else {
                                None
                            }
                        })
                        .for_each(|x| moves.push(x));
                    moves
                }
                (PieceType::Pawn, PlayerType::White) => {
                    let mut moves = vec![];
                    if let Some(final_pos) = ChessPosition::new(pos.file, pos.rank + 1) {
                        if self.board.get_piece_at_pos(&final_pos).is_none() {
                            moves.push(ChessMove::new(
                                ChessMoveKind::Normal,
                                ChessMoveVector::new(*pos, final_pos),
                                piece,
                            ));
                            if let Some(final_pos) = ChessPosition::new(pos.file, pos.rank + 2) {
                                if self.board.get_piece_at_pos(&final_pos).is_none()
                                    && pos.rank == 1
                                {
                                    moves.push(ChessMove::new(
                                        ChessMoveKind::Normal,
                                        ChessMoveVector::new(*pos, final_pos),
                                        piece,
                                    ));
                                }
                            }
                        }
                    }

                    // dbg!(&moves, pos, self.en_passant_target_square);
                    [(-1, 1), (1, 1)]
                        .iter()
                        .filter_map(|(file, rank)| {
                            Some(ChessPosition::new(pos.file + file, pos.rank + rank)?)
                        })
                        .filter_map(|final_pos| {
                            if self
                                .board
                                .get_piece_at_pos(&final_pos)
                                .is_some_and(|x| x.player_kind != self.active_color)
                            {
                                Some(ChessMove::new(
                                    ChessMoveKind::Take(piece.piece_kind),
                                    ChessMoveVector::new(*pos, final_pos),
                                    piece,
                                ))
                            } else if final_pos == self.en_passant_target_square? {
                                Some(ChessMove::new(
                                    ChessMoveKind::EnPassant(self.en_passant_target_square?),
                                    ChessMoveVector::new(*pos, final_pos),
                                    piece,
                                ))
                            } else {
                                None
                            }
                        })
                        .for_each(|x| moves.push(x));
                    moves
                }
                (PieceType::Rook, _) => self.get_pseudo_legal_moves_from_offsets(
                    pos,
                    &[(1, 0), (0, -1), (-1, 0), (0, 1)],
                    &piece,
                    8,
                ),
                (PieceType::Knight, _) => self.get_pseudo_legal_moves_from_offsets(
                    pos,
                    &[
                        (1, 2),
                        (2, 1),
                        (2, -1),
                        (1, -2),
                        (-1, -2),
                        (-2, -1),
                        (-2, 1),
                        (-1, 2),
                    ],
                    &piece,
                    1,
                ),
                (PieceType::Bishop, _) => self.get_pseudo_legal_moves_from_offsets(
                    pos,
                    &[(1, 1), (1, -1), (-1, -1), (-1, 1)],
                    &piece,
                    8,
                ),
                (PieceType::Queen, _) => self.get_pseudo_legal_moves_from_offsets(
                    pos,
                    &[
                        (1, 0),
                        (0, -1),
                        (-1, 0),
                        (0, 1),
                        (1, 1),
                        (1, -1),
                        (-1, -1),
                        (-1, 1),
                    ],
                    &piece,
                    8,
                ),
                (PieceType::King, _) => {
                    let mut moves = self.get_pseudo_legal_moves_from_offsets(
                        pos,
                        &[
                            (1, 0),
                            (0, -1),
                            (-1, 0),
                            (0, 1),
                            (1, 1),
                            (1, -1),
                            (-1, -1),
                            (-1, 1),
                        ],
                        &piece,
                        1,
                    );
                    match piece.player_kind {
                        PlayerType::White => {
                            if self.castling_availability.0
                                && self.board.0[0][1].is_none()
                                && self.board.0[0][2].is_none()
                            {
                                moves.push(ChessMove::new(
                                    ChessMoveKind::Castle(CastleType::Short),
                                    ChessMoveVector::new(*pos, ChessPosition::new(1, 0).unwrap()),
                                    piece,
                                ));
                            }
                            if self.castling_availability.1
                                && self.board.0[0][4].is_none()
                                && self.board.0[0][5].is_none()
                                && self.board.0[0][6].is_none()
                            {
                                moves.push(ChessMove::new(
                                    ChessMoveKind::Castle(CastleType::Long),
                                    ChessMoveVector::new(*pos, ChessPosition::new(5, 0).unwrap()),
                                    piece,
                                ));
                            }
                        }
                        PlayerType::Black => {
                            if self.castling_availability.2
                                && self.board.0[7][1].is_none()
                                && self.board.0[7][2].is_none()
                            {
                                moves.push(ChessMove::new(
                                    ChessMoveKind::Castle(CastleType::Short),
                                    ChessMoveVector::new(*pos, ChessPosition::new(1, 7).unwrap()),
                                    piece,
                                ));
                            }
                            if self.castling_availability.1
                                && self.board.0[7][4].is_none()
                                && self.board.0[7][5].is_none()
                                && self.board.0[7][6].is_none()
                            {
                                moves.push(ChessMove::new(
                                    ChessMoveKind::Castle(CastleType::Long),
                                    ChessMoveVector::new(*pos, ChessPosition::new(5, 7).unwrap()),
                                    piece,
                                ));
                            }
                        }
                    }
                    moves
                }
            },
            None => {
                vec![]
            }
        }
    }
    fn get_pseudo_legal_moves_from_offsets(
        &self,
        loc: &ChessPosition,
        offsets: &[(i32, i32)],
        piece: &Piece,
        max_jump: i32,
    ) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        for offset in offsets {
            for i in 1..=max_jump {
                if let Some(to_pos) =
                    self.move_possibility(loc, &(offset.0 * i, offset.1 * i), &piece.player_kind)
                {
                    if let Some(to_piece) = self.board.get_piece_at_pos(&to_pos) {
                        moves.push(ChessMove::new(
                            ChessMoveKind::Take(to_piece.piece_kind),
                            ChessMoveVector::new(*loc, to_pos),
                            piece.clone(),
                        ));
                        break;
                    } else {
                        moves.push(ChessMove::new(
                            ChessMoveKind::Normal,
                            ChessMoveVector::new(*loc, to_pos),
                            piece.clone(),
                        ));
                    }
                } else {
                    break;
                }
            }
        }
        moves
    }
    fn move_possibility(
        &self,
        loc: &ChessPosition,
        offset: &(i32, i32),
        player_kind: &PlayerType,
    ) -> Option<ChessPosition> {
        let new_pos = loc.add_offset(offset)?;
        if self.board.get_piece_at_pos(&new_pos).is_none()
            || self
                .board
                .get_piece_at_pos(&new_pos)
                .is_some_and(|x| (x.player_kind != *player_kind))
        {
            Some(new_pos)
        } else {
            None
        }
    }
    pub fn make_move(
        &mut self,
        from_pos: &ChessPosition,
        to_pos: &ChessPosition,
    ) -> Option<ChessMove> {
        let moves = self.generate_pseudo_legal_moves();
        for chess_move in moves {
            if (chess_move.move_vector.from_pos == *from_pos)
                && (chess_move.move_vector.to_pos == *to_pos)
            {
                return Some(chess_move);
            }
        }
        None
    }
    pub fn do_move(&mut self, chess_move: &ChessMove) -> MoveResult {
        // impliment chess checks and making a move that will result in check is not a move
        if chess_move.move_vector.from_pos != chess_move.move_vector.to_pos {
            if (if !self.recurrance {
                self.generate_legal_moves_for_pos(&chess_move.move_vector.from_pos)
            } else {
                self.generate_pseudo_legal_moves_for_pos(&chess_move.move_vector.from_pos)
            })
            .contains(&chess_move)
            {
                self.board.perform_move(chess_move);
                if self.active_color == PlayerType::Black {
                    self.full_move_no += 1;
                }
                if self.active_color == PlayerType::White {
                    self.half_move_clock += 1;
                }
                if chess_move.move_vector.from_pos.file == 0
                    && chess_move.move_vector.from_pos.rank == 0
                {
                    self.castling_availability.0 = false;
                }
                if chess_move.move_vector.from_pos.file == 3
                    && chess_move.move_vector.from_pos.rank == 0
                {
                    self.castling_availability.0 = false;
                    self.castling_availability.1 = false;
                }

                if chess_move.move_vector.from_pos.file == 7
                    && chess_move.move_vector.from_pos.rank == 0
                {
                    self.castling_availability.1 = false;
                }
                if chess_move.move_vector.from_pos.file == 0
                    && chess_move.move_vector.from_pos.rank == 7
                {
                    self.castling_availability.2 = false;
                }
                if chess_move.move_vector.from_pos.file == 3
                    && chess_move.move_vector.from_pos.rank == 7
                {
                    self.castling_availability.2 = false;
                    self.castling_availability.3 = false;
                }

                if chess_move.move_vector.from_pos.file == 7
                    && chess_move.move_vector.from_pos.rank == 7
                {
                    self.castling_availability.3 = false;
                }
                self.active_color = match self.active_color {
                    PlayerType::Black => PlayerType::White,
                    PlayerType::White => PlayerType::Black,
                };
                if self.recurrance == false {
                    dbg!(self.generate_legal_moves().len());
                    if self.is_stale_mate() {
                        println!("Stalemate");
                        return MoveResult::GameEnd(GameEnd::StaleMate);
                    } else if self.is_check_mate() {
                        println!("Checkmate");
                        return MoveResult::GameEnd(GameEnd::Checkmate);
                    }
                }
            }
        } else {
            return MoveResult::InvalidMove;
        }

        MoveResult::Normal
    }
    pub fn generate_legal_moves_for_pos(&self, pos: &ChessPosition) -> Vec<ChessMove> {
        self.generate_pseudo_legal_moves_for_pos(pos)
            .iter()
            .filter_map(|chess_move| {
                let mut new_state: GameState = self.clone();
                new_state.recurrance = true;
                new_state.do_move(chess_move);
                if !new_state.is_prev_in_check() {
                    Some(chess_move.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn is_prev_in_check(&self) -> bool {
        let king_pos = self
            .board
            .0
            .iter()
            .enumerate()
            .find_map(|(rank, row)| {
                row.iter().enumerate().find_map(|(file, piece)| {
                    if piece.is_some_and(|piece| {
                        piece.piece_kind == PieceType::King
                            && piece.player_kind != self.active_color
                    }) {
                        Some(ChessPosition::new(file as i32, rank as i32).unwrap())
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        // dbg!(king_pos);
        self.generate_pseudo_legal_moves()
            .iter()
            .map(|chess_move| chess_move.move_vector.to_pos)
            .any(|pos| pos == king_pos)
    }
    // TODO : Rewrite this so it is always right
    pub fn is_check(&self) -> bool {
        let king_pos = self
            .board
            .0
            .iter()
            .enumerate()
            .find_map(|(rank, row)| {
                row.iter().enumerate().find_map(|(file, piece)| {
                    if piece.is_some_and(|piece| {
                        piece.piece_kind == PieceType::King
                            && piece.player_kind == self.active_color
                    }) {
                        Some(ChessPosition::new(file as i32, rank as i32).unwrap())
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        // dbg!(king_pos);
        let mut new_state: GameState = self.clone();
        // TODO : use undo here when made
        new_state.active_color = match self.active_color {
            PlayerType::Black => PlayerType::White,
            PlayerType::White => PlayerType::Black,
        };
        dbg!(&new_state.generate_pseudo_legal_moves().len());
        new_state
            .generate_pseudo_legal_moves()
            .iter()
            .map(|chess_move| chess_move.move_vector.to_pos)
            .any(|pos| pos == king_pos)
    }
    pub fn is_check_mate(&self) -> bool {
        // dbg!("no moves are ", self.generate_legal_moves().len());
        (self.generate_legal_moves().len() == 0) && self.is_check()
    }
    pub fn is_stale_mate(&self) -> bool {
        (self.generate_legal_moves().len() == 0) && !self.is_check()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MoveResult {
    GameEnd(GameEnd),
    Normal,
    InvalidMove,
}
