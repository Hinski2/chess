use crate::BoardState;
use crate::generate_moves::king::KING_ATTACK;
use crate::generate_moves::knight::KNIGHT_ATTACK;
use crate::piece_move::MoveFlag;
use super::piece_move::PieceMove;
use super::{Color, Piece, PieceColor};

pub struct Board {
    pub(crate) side_to_move: Color,

    pub(crate) bitboard: [[u64; 2]; 6],
    pub(crate) occupied:  [u64; 2],
    pub(crate) pieces:   [PieceColor; 64],

    pub(crate) board_state: BoardState,
}

impl Board {
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
            pieces: pieces,
            board_state: BoardState::new(),
        }
    }  

    // generates all valid moves
    pub fn generate_all_moves(&mut self) -> Vec<PieceMove> {
        Piece::ALL.iter()
            .flat_map(|piece_type| self.generate_piece_move(piece_type))
            .collect()
    }    

    // execute move, update board state and swiches sides
    pub fn do_move(&mut self, piece_move: &PieceMove) {
        match piece_move.flag {
            MoveFlag::PromoteToQueenAndCapture | MoveFlag::PromoteToRookAndCapture | MoveFlag::PromoteToBishopAndCapture
                | MoveFlag::PromoteToKnightAndCapture => {
                    self.handle_capture(piece_move);
                    self.handle_promotion(piece_move);
                    self.board_state.en_passant = None;
                },
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToRook | MoveFlag::PromoteToBishop
                | MoveFlag::PromoteToKnight => {
                    self.handle_move(piece_move);
                    self.handle_promotion(piece_move);
                    self.board_state.en_passant = None;
                },
            MoveFlag::Capture => {
                self.handle_capture(piece_move);
                self.board_state.en_passant = None;
            },
            MoveFlag::EnPassantCapture => {
                self.handle_en_passant_capture(piece_move);
                self.board_state.en_passant = None;
            },
            MoveFlag::Castling => {
                self.handle_castle(piece_move);
                self.board_state.en_passant = None;
            },
            MoveFlag::DoublePawnPush => {
                self.handle_double_pawn_push(piece_move);
            },
            MoveFlag::Normal => {
                self.handle_move(piece_move);
                self.board_state.en_passant = None;
            }
        }

        self.side_to_move = self.side_to_move.get_opposite();
    }

    pub fn undo_move(&mut self, piece_move: &PieceMove, board_state: BoardState) {
       todo!(); 
    }

    // generates piece (not neccecary valid) moves for a piece
    fn generate_piece_move(&mut self, piece_type: &Piece) -> Vec<PieceMove> {
        let positions = self.bitboard[*piece_type as usize][self.side_to_move as usize].clone();

        match (piece_type, self.side_to_move) {
            (Piece::Pawn, Color::White) => self.generate_pawn_moves_white(positions),
            (Piece::Pawn, Color::Black) => self.generate_pawn_moves_black(positions),

            (Piece::Knight, Color::White) => self.generate_knight_moves_white(positions),
            (Piece::Knight, Color::Black) => self.generate_knight_moves_black(positions),

            (Piece::Bishop, Color::White) => self.generate_bishop_moves_white(positions),
            (Piece::Bishop, Color::Black) => self.generate_bishop_moves_black(positions),

            (Piece::Rook, Color::White) => self.generate_rook_moves_white(positions),
            (Piece::Rook, Color::Black) => self.generate_rook_moves_black(positions),

            (Piece::Queen, Color::White) => self.generate_queen_moves_white(positions),
            (Piece::Queen, Color::Black) => self.generate_queen_moves_black(positions),

            (Piece::King, Color::White) => self.generate_king_moves_white(positions),
            (Piece::King, Color::Black) => self.generate_king_moves_black(positions),
        }.into_iter()
            .filter(|piece_move| self.validate_move_filter(piece_move))
            .collect::<Vec<PieceMove>>()
    }

    fn validate_move_filter(&mut self, piece_move: &PieceMove) -> bool {
        let board_state_copy = self.board_state;

        self.do_move(&piece_move);
        let opposite_king= self.bitboard[Piece::King as usize][self.side_to_move.get_opposite() as usize].trailing_zeros() as u8;
        let is_attacked = self.is_tile_attacked(opposite_king);
        self.undo_move(&piece_move, board_state_copy);
        is_attacked
    }

    fn is_tile_attacked(&self, idx: u8) -> bool {
        // checks if current color can attack piece on tile

        let x = (idx / 8) as i32;
        let y = (idx % 8) as i32;

        let empty = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let our_pieces = self.occupied[self.side_to_move as usize];

        // diagonal check 
        let shifts = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
        for shift in shifts {
            let mut _pos = (x + shift.0, y + shift.1);

            while 0 <= _pos.0 && _pos.0 < 8 && 0 <= _pos.1 && _pos.1 < 8 {
                let to = _pos.0 * 8 + _pos.1;

                if our_pieces & (1 << to) > 0 {
                    if (self.bitboard[Piece::Bishop as usize][self.side_to_move as usize] 
                        | self.bitboard[Piece::Queen as usize][self.side_to_move as usize]) & (1 << to) > 0 {
                        return true;
                    } else {
                        break;
                    }
                } else if empty & (1 << to) > 0 {
                    _pos.0 += shift.0; 
                    _pos.1 += shift.1;
                } else {
                    break;
                }
            }
        }

        // straight 
        let shifts = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for shift in shifts {
            let mut _pos = (x + shift.0, y + shift.1);

            while 0 <= _pos.0 && _pos.0 < 8 && 0 <= _pos.1 && _pos.1 < 8 {
                let to = _pos.0 * 8 + _pos.1;

                if our_pieces & (1 << to) > 0 {
                    if (self.bitboard[Piece::Rook as usize][self.side_to_move as usize] 
                        | self.bitboard[Piece::Queen as usize][self.side_to_move as usize]) & (1 << to) > 0{
                        return true;
                    } else {
                        break;
                    }
                } else if empty & (1 << to) > 0 {
                    _pos.0 += shift.0; 
                    _pos.1 += shift.1;
                } else {
                    break;
                }
            }
        }

        // knight
        if KNIGHT_ATTACK[idx as usize] & self.bitboard[Piece::Knight as usize][self.side_to_move as usize] > 0 {
            return true;
        }

        // pawn
        match self.side_to_move {
            Color::Black if x < 7 => {
                if (y != 0 && (1 << (idx + 7)) & self.bitboard[Piece::Pawn as usize][Color::Black as usize] > 0) 
                    || (y != 7 && (1 << (idx + 9)) & self.bitboard[Piece::Pawn as usize][Color::Black as usize] > 0) {
                    return true
                }
            },
            Color::White if x > 0 => {
                if (y != 7 && (1 << (idx - 7)) & self.bitboard[Piece::Pawn as usize][Color::White as usize] > 0) 
                    || (y != 0 && (1 << (idx - 9)) & self.bitboard[Piece::Pawn as usize][Color::White as usize] > 0) {
                    return true
                }
            },
            _ => (),
        }

        // king
        if KING_ATTACK[idx as usize] & self.bitboard[Piece::King as usize][self.side_to_move as usize] > 0 {
            return true;
        }

        return false;
    }
}

