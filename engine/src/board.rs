use super::piece_move::PieceMove;
use super::{Color, Piece, PieceColor};

pub struct Board {
    pub(crate) side_to_move: Color,

    pub(crate) bitboard: [[u64; 2]; 6],
    pub(crate) occupied:  [u64; 2],
    pub(crate) pieces:   [PieceColor; 64],

    pub(crate) en_passant: Option<u8>,     // idx where we can attack with en passant
    pub(crate) castle_rights: u8,          // 0: white left, 1: white right, 2: back left, 3: back right
}

impl Board {
// public
    pub fn new() -> Board {

        const WHITE: usize = Color::White as usize;
        const BLACK: usize = Color::Black as usize;

        let mut bitboard = [[0u64; 2]; 6];

        bitboard[Piece::Pawn as usize][WHITE] = 0x0000_0000_0000_ff00;
        bitboard[Piece::Pawn as usize][BLACK] = 0x00ff_0000_0000_0000;

        bitboard[Piece::Knight as usize][WHITE] = 0x0000_0000_0000_0042;
        bitboard[Piece::Knight as usize][BLACK] = 0x4200_0000_0000_0000;

        bitboard[Piece::Bishop as usize][WHITE] = 0x0000_0000_0000_0024;
        bitboard[Piece::Bishop as usize][BLACK] = 0x2400_0000_0000_0000;

        bitboard[Piece::Rook as usize][WHITE] = 0x0000_0000_0000_0081;
        bitboard[Piece::Rook as usize][BLACK] = 0x8100_0000_0000_0000;

        bitboard[Piece::Queen as usize][WHITE] = 0x0000_0000_0000_0008;
        bitboard[Piece::Queen as usize][BLACK] = 0x0800_0000_0000_0000;

        bitboard[Piece::King as usize][WHITE] = 0x0000_0000_0000_0010;
        bitboard[Piece::King as usize][BLACK] = 0x1000_0000_0000_0000;

        use PieceColor::*;
        let pieces: [PieceColor; 64] = [
            WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook, // rank 1, 0 - 7
            WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn,
            BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook, // rank 8, 56 - 63
        ];

        Board {
            side_to_move: Color::White,
            bitboard: bitboard,
            occupied: [0x0000_0000_0000_ffff, 0xffff_0000_0000_0000],
            en_passant: Option::None,
            pieces: pieces,
            castle_rights: 0x0F,
        }
    }  

    pub fn generate_moves(&self) -> Vec<PieceMove> {
        Piece::ALL.iter()
            .flat_map(|&piece_type| self.generate_piece_move(piece_type))
            .collect()
    }    

    pub fn do_move(&mut self, piece_move: &PieceMove) {
       todo!(); 
    }

    pub fn undo_move(&mut self, piece_move: &PieceMove) {
       todo!(); 
    }

// private
    fn generate_piece_move(&self, piece_type: Piece) -> Vec<PieceMove> {
        let positions = self.bitboard[piece_type as usize][self.side_to_move as usize].clone();

        return match (piece_type, self.side_to_move) {
            (Piece::Pawn, Color::White) => self.generate_pawn_moves_white(positions),
            (Piece::Pawn, Color::Black) => self.generate_pawn_moves_black(positions),
            Piece::Knight => self.generate_knight_moves(positions),
            Piece::Bishop => self.generate_bishop_moves(positions),
            Piece::Rook => self.generate_rook_moves(positions),
            Piece::Queen => self.generate_queen_moves(positions),
            Piece::King => self.generate_king_moves(positions),
        }
    }
}

