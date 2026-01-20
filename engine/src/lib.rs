pub mod piece_move;
pub mod board;
pub mod generate_moves;
pub mod move_handlers;
pub mod undo_move_handlers;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline(always)]
    pub fn get_opposite(&self) -> Color {
        if *self == Color::White { Color::Black } else { Color::White }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    None, 
    WhitePawn, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing,
    BlackPawn, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing,
}

impl PieceColor {
    pub fn new(piece: Piece, color: Color) -> PieceColor {
        match (piece, color) {
            (Piece::Pawn, Color::White) => PieceColor::WhitePawn,
            (Piece::Knight, Color::White) => PieceColor::WhiteKnight,
            (Piece::Bishop, Color::White) => PieceColor::WhiteBishop,
            (Piece::Rook, Color::White) => PieceColor::WhiteRook,
            (Piece::Queen, Color::White) => PieceColor::WhiteQueen,
            (Piece::King, Color::White) => PieceColor::WhiteKing,

            (Piece::Pawn, Color::Black) => PieceColor::BlackPawn,
            (Piece::Knight, Color::Black) => PieceColor::BlackKnight,
            (Piece::Bishop, Color::Black) => PieceColor::BlackBishop,
            (Piece::Rook, Color::Black) => PieceColor::BlackRook,
            (Piece::Queen, Color::Black) => PieceColor::BlackQueen,
            (Piece::King, Color::Black) => PieceColor::BlackKing,
        }
    }

    pub fn extract_piece(&self) -> Piece {
        use PieceColor::*;

        match self {
            WhitePawn | BlackPawn => Piece::Pawn,
            WhiteKnight | BlackKnight => Piece::Knight,
            WhiteBishop | BlackBishop => Piece::Bishop,
            WhiteRook | BlackRook => Piece::Rook, 
            WhiteQueen | BlackQueen => Piece::Queen,
            WhiteKing | BlackKing => Piece::King,
            None => unreachable!("You try to extract Piece from PieceColor::None"),
        }
    }

    pub fn extract_color(&self) -> Color {
        use PieceColor::*;

        match self {
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => Color::White,
            _ => Color::Black,
        }
    }
}

impl Piece {
    const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];
}


#[derive(Clone, Copy)]
pub struct BoardState {
    castle_rights: u8,
    pub(crate) captured_piece_type: Option<Piece>,  // if we captured a piece in the last move thats the type of the piece
    pub(crate) en_passant: Option<u8>,     // idx where we can attack with en passant
}

impl BoardState {
    pub fn new() -> Self {
        BoardState { castle_rights: 0b1111, captured_piece_type: None, en_passant: None }
    }

    pub(crate) fn castle_rights_white_left(&self) -> bool {
        self.castle_rights & (1 << 0) > 0
    }
    pub(crate) fn castle_rights_white_right(&self) -> bool {
        self.castle_rights & (1 << 1) > 0
    }
    pub(crate) fn castle_rights_black_left(&self) -> bool {
        self.castle_rights & (1 << 2) > 0
    }
    pub(crate) fn castle_rights_black_right(&self) -> bool {
        self.castle_rights & (1 << 3) > 0
    }
}
