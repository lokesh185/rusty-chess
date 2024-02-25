use crate::common::logic::{Piece, PieceType, PlayerType};

fn verify_fen_board(fen: &str) -> bool {
    let (mut file, mut rank) = (0, 7);
    for i in fen.chars() {
        if i == '/' {
            file = 0;
            rank -= 1;
            if rank < 0 {
                // println!("f1");

                return false;
            }
            continue;
        }
        if i.is_numeric() {
            file += i.to_digit(10).unwrap_or(0);
        } else if i.is_ascii_alphabetic() {
            file += 1;
        } else {
            // println!("f2");
            return false;
        }
        if file > 8 {
            return false;
        }
    }
    true
}

fn pos_from_algebraic_notation(pos: &str) -> Option<(usize, usize)> {
    if pos.len() == 2 {
        let mut it = pos.chars().into_iter();
        Some((
            match it.next() {
                Some('a') => 0,
                Some('b') => 1,
                Some('c') => 2,
                Some('d') => 3,
                Some('e') => 4,
                Some('f') => 5,
                Some('g') => 6,
                Some('h') => 7 as usize,
                _ => {
                    return None;
                }
            },
            match it.next() {
                Some(num) => match num.to_digit(10) {
                    Some(nu) => nu as usize,
                    None => {
                        return None;
                    }
                },
                None => {
                    return None;
                }
            },
        ))
    } else {
        None
    }
}
pub struct GameData {
    pub table: [[Option<Piece>; 8]; 8],
    pub active_color: PlayerType,
    pub castling_availability: (bool, bool, bool, bool),
    // 0 -> white castle king side , 1 -> white castle queen side
    // 2 -> black castle king side , 3 -> black castle queen side
    pub en_passant_target_square: Option<(usize, usize)>,
    pub halfmove_clock: u32, //The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.
    pub fullmove_number: u32, //  The number of the full moves. It starts at 1 and is incremented after Black's move.
}
impl GameData {
    pub fn from_fen(fen: &str) -> Option<Self> {
        let (mut file, mut rank) = (0, 7);
        let mut table: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        let mut sp = fen.split_ascii_whitespace();
        let fen_board = match sp.next() {
            Some(ele) => ele,
            None => {
                return None;
            }
        };
        let cur_player = match sp.next() {
            Some(ele) => match ele.len() {
                1 => {
                    let x = ele.chars().next();
                    // dbg!(&x);
                    match x {
                        Some('w') => PlayerType::White,
                        Some('b') => PlayerType::Black,
                        _ => {
                            return None;
                        }
                    }
                }
                _ => {
                    return None;
                }
            },
            None => {
                return None;
            }
        };
        let mut castle = (false, false, false, false);

        match sp.next() {
            Some(ele) => {
                for ch in ele.chars() {
                    match ch {
                        'k' => {
                            castle.2 = true;
                        }
                        'q' => {
                            castle.3 = true;
                        }
                        'K' => {
                            castle.0 = true;
                        }
                        'Q' => {
                            castle.1 = true;
                        }
                        _ => {}
                    }
                }
            }
            None => {
                return None;
            }
        };

        let en_pass = pos_from_algebraic_notation(sp.next()?);

        let half_move_no = sp.next()?.parse::<u32>().ok()?;

        let move_no = sp.next()?.parse::<u32>().ok()?;

        if verify_fen_board(fen_board) {
            for i in fen_board.chars() {
                if i == '/' {
                    file = 0;
                    rank -= 1;
                } else if i.is_numeric() {
                    file += i.to_digit(10).unwrap_or(0);
                } else {
                    let player_type = if i.is_uppercase() {
                        PlayerType::White
                    } else {
                        PlayerType::Black
                    };
                    let piece = match i.to_ascii_lowercase() {
                        'k' => Some(Piece::new(PieceType::King, player_type)),
                        'p' => Some(Piece::new(PieceType::Pawn, player_type)),
                        'n' => Some(Piece::new(PieceType::Knight, player_type)),
                        'b' => Some(Piece::new(PieceType::Bishop, player_type)),
                        'r' => Some(Piece::new(PieceType::Rook, player_type)),
                        'q' => Some(Piece::new(PieceType::Queen, player_type)),
                        _ => None,
                    };
                    table[rank as usize][(7 - file) as usize] = piece;
                    file += 1;
                }
            }
            Some(Self {
                table: table,
                active_color: cur_player,
                castling_availability: castle,
                en_passant_target_square: en_pass,
                halfmove_clock: half_move_no,
                fullmove_number: move_no,
            })
        } else {
            return None;
        }
    }
}
