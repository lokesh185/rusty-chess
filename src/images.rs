use egui::{include_image, Image, Vec2};
// use crate::api::ChessApi;
use crate::common::logic::{Piece, PieceType, PlayerType};
#[derive(Clone)]
pub struct PieceImages<'a> {
    black_pawn_image: Image<'a>,
    white_pawn_image: Image<'a>,

    black_bishop_image: Image<'a>,
    white_bishop_image: Image<'a>,

    black_knight_image: Image<'a>,
    white_knight_image: Image<'a>,

    black_rook_image: Image<'a>,
    white_rook_image: Image<'a>,

    black_queen_image: Image<'a>,
    white_queen_image: Image<'a>,

    black_king_image: Image<'a>,
    white_king_image: Image<'a>,

    pub image_size: Vec2,
}

impl<'a> Default for PieceImages<'a> {
    fn default() -> Self {
        Self {
            black_pawn_image: Image::new(include_image!("../assets/imgs/Chess_pdt60.png")),
            white_pawn_image: Image::new(include_image!("../assets/imgs/Chess_plt60.png")),
            black_bishop_image: Image::new(include_image!("../assets/imgs/Chess_bdt60.png")),
            white_bishop_image: Image::new(include_image!("../assets/imgs/Chess_blt60.png")),
            black_knight_image: Image::new(include_image!("../assets/imgs/Chess_ndt60.png")),
            white_knight_image: Image::new(include_image!("../assets/imgs/Chess_nlt60.png")),
            black_rook_image: Image::new(include_image!("../assets/imgs/Chess_rdt60.png")),
            white_rook_image: Image::new(include_image!("../assets/imgs/Chess_rlt60.png")),
            black_queen_image: Image::new(include_image!("../assets/imgs/Chess_qdt60.png")),
            white_queen_image: Image::new(include_image!("../assets/imgs/Chess_qlt60.png")),
            black_king_image: Image::new(include_image!("../assets/imgs/Chess_kdt60.png")),
            white_king_image: Image::new(include_image!("../assets/imgs/Chess_klt60.png")),
            image_size: Vec2::new(60., 60.),
        }
    }
}
impl<'a> PieceImages<'a> {
    pub fn get(self: &Self, piece: &Piece) -> &Image<'a> {
        match piece.player_kind {
            PlayerType::White => match piece.piece_kind {
                PieceType::Pawn => &self.white_pawn_image,
                PieceType::Bishop => &self.white_bishop_image,
                PieceType::Knight => &self.white_knight_image,
                PieceType::Rook => &self.white_rook_image,
                PieceType::Queen => &self.white_queen_image,
                PieceType::King => &self.white_king_image,
            },
            PlayerType::Black => match piece.piece_kind {
                PieceType::Pawn => &self.black_pawn_image,
                PieceType::Bishop => &self.black_bishop_image,
                PieceType::Knight => &self.black_knight_image,
                PieceType::Rook => &self.black_rook_image,
                PieceType::Queen => &self.black_queen_image,
                PieceType::King => &self.black_king_image,
            },
        }
    }
}
