use core::fmt;

use super::BoardState;
use super::generate_moves::king::KING_ATTACK;
use super::generate_moves::knight::KNIGHT_ATTACK;
use super::move_handlers::{EN_PASSANT_HSH, SIDE_TO_MOVE_HSH};
use super::piece_move::MoveFlag;
use super::piece_move::PieceMove;
use super::{Color, Piece, PieceColor};

#[derive(Clone)]
pub struct Board {
    pub(super) side_to_move: Color,

    pub bitboard: [[u64; 2]; 6],
    pub occupied:  [u64; 2],
    pub(crate) pieces:   [PieceColor; 64],

    pub(super) board_state: BoardState,
    pub(super) hsh: u64,
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

        let mut board = Board {
            side_to_move: Color::White,
            bitboard: bitboard,
            occupied: [0x0000_0000_0000_ffff, 0xffff_0000_0000_0000],
            pieces: pieces,
            board_state: BoardState::new(),
            hsh: 0u64, // temp
        };

        board.hsh = board.compute_full_hsh();
        board
    }  

    pub fn get_size_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn get_board_state(&self) -> BoardState {
        self.board_state.clone()
    }

    pub fn get_board_hsh(&self) -> u64 {
        self.hsh
    }

    /// generates all valid moves
    pub fn generate_all_moves(&mut self) -> Vec<PieceMove> {
        Piece::ALL.iter()
            .flat_map(|piece_type| self.generate_piece_move(piece_type))
            .collect()
    }    

    /// execute move, update board state and swiches sides
    pub fn do_move(&mut self, piece_move: &PieceMove) {
        if let Some(ep_idx) = self.board_state.en_passant {
            self.hsh ^= EN_PASSANT_HSH[(ep_idx % 8) as usize];
            self.board_state.en_passant = None;
        }

        self.hsh ^= self.calculate_castle_hsh();
        match piece_move.flag {
            MoveFlag::PromoteToQueenAndCapture | MoveFlag::PromoteToRookAndCapture | MoveFlag::PromoteToBishopAndCapture
                | MoveFlag::PromoteToKnightAndCapture => {
                    self.handle_capture(piece_move);
                    self.handle_promotion(piece_move);
                },
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToRook | MoveFlag::PromoteToBishop
                | MoveFlag::PromoteToKnight => {
                    self.handle_move(piece_move);
                    self.handle_promotion(piece_move);
                },
            MoveFlag::Capture => {
                self.handle_capture(piece_move);
            },
            MoveFlag::EnPassantCapture => {
                self.handle_en_passant_capture(piece_move);
            },
            MoveFlag::Castling => {
                self.handle_castle(piece_move);
            },
            MoveFlag::DoublePawnPush => {
                self.handle_double_pawn_push(piece_move);
            },
            MoveFlag::Normal => {
                self.handle_move(piece_move);
            }
            MoveFlag::None => unreachable!(),
        }
        self.hsh ^= self.calculate_castle_hsh();
        if let Some(ep_idx) = self.board_state.en_passant {
            self.hsh ^= EN_PASSANT_HSH[(ep_idx % 8) as usize];
        }

        self.side_to_move = self.side_to_move.get_opposite();
        self.hsh ^= SIDE_TO_MOVE_HSH;
    }

    /// undoes the last move
    pub fn undo_move(&mut self, piece_move: &PieceMove, board_state: BoardState, captured_piece_type: Option<Piece>) {
        // switch side
        self.side_to_move = self.side_to_move.get_opposite();
        self.hsh ^= SIDE_TO_MOVE_HSH;

        // remove ep, castle rights
        if let Some(ep_idx) = self.board_state.en_passant {
            self.hsh ^= EN_PASSANT_HSH[(ep_idx % 8) as usize];
        }
        self.hsh ^= self.calculate_castle_hsh();

        // return to the old state
        self.board_state = board_state;

        match piece_move.flag {
            MoveFlag::PromoteToQueenAndCapture | MoveFlag::PromoteToRookAndCapture | MoveFlag::PromoteToBishopAndCapture
                | MoveFlag::PromoteToKnightAndCapture => {
                    self.handle_undo_promotion(piece_move);
                    self.handle_undo_capture(piece_move, &captured_piece_type.unwrap())
                },
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToRook | MoveFlag::PromoteToBishop
                | MoveFlag::PromoteToKnight => {
                    self.handle_undo_promotion(piece_move);
                    self.handle_undo_move(piece_move);
                },
            MoveFlag::Capture => {
                self.handle_undo_capture(piece_move, &captured_piece_type.unwrap());
            },
            MoveFlag::EnPassantCapture => {
                self.handle_undo_en_passant_capture(piece_move);
            },
            MoveFlag::Castling => {
                self.handle_undo_castle(piece_move);
            },
            MoveFlag::DoublePawnPush => {
                self.handle_undo_double_pawn_push(piece_move);
            },
            MoveFlag::Normal => {
                self.handle_undo_move(piece_move);
            }
            MoveFlag::None => unreachable!(),
        }

        // add castle rights and ep
        self.hsh ^= self.calculate_castle_hsh();
        if let Some(ep_idx) = self.board_state.en_passant {
            self.hsh ^= EN_PASSANT_HSH[(ep_idx % 8) as usize];
        }
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
        let board_state_copy= self.board_state.clone();
        let captured_piece_type = self.pieces[piece_move.to as usize].try_extract_piece();
        
        // println!("do: {:?}", &piece_move); // !!!!!!1
        self.do_move(&piece_move);
        // println!("{}", self); // !!!!!!!1

        let opposite_king= self.bitboard[Piece::King as usize][self.side_to_move.get_opposite() as usize].trailing_zeros() as u8;
        let is_attacked = self.is_tile_attacked(opposite_king);


        // println!("undo: {:?}", &piece_move); // !!!!!!1
        self.undo_move(&piece_move, board_state_copy, captured_piece_type);
        // println!("{}", self); // !!!!!!!1

        !is_attacked
    }
    
    
    pub fn is_checked(&mut self) -> bool {
        let king_idx = self.bitboard[Piece::King as usize][self.side_to_move as usize].trailing_zeros() as u8;

        self.side_to_move = self.side_to_move.get_opposite();
        let ans = self.is_tile_attacked(king_idx);
        self.side_to_move = self.side_to_move.get_opposite();
        ans
    }

    /// checks if current color can attack piece on tile
    fn is_tile_attacked(&self, idx: u8) -> bool {
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

                if our_pieces & (1u64 << to) > 0 {
                    if (self.bitboard[Piece::Bishop as usize][self.side_to_move as usize] 
                        | self.bitboard[Piece::Queen as usize][self.side_to_move as usize]) & (1u64 << to) > 0 {
                        return true;
                    } else {
                        break;
                    }
                } else if empty & (1u64 << to) > 0 {
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

                if our_pieces & (1u64 << to) > 0 {
                    if (self.bitboard[Piece::Rook as usize][self.side_to_move as usize] 
                        | self.bitboard[Piece::Queen as usize][self.side_to_move as usize]) & (1u64 << to) > 0{
                        return true;
                    } else {
                        break;
                    }
                } else if empty & (1u64 << to) > 0 {
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
                if (y != 0 && (1u64 << (idx + 7)) & self.bitboard[Piece::Pawn as usize][Color::Black as usize] > 0) 
                    || (y != 7 && (1u64 << (idx + 9)) & self.bitboard[Piece::Pawn as usize][Color::Black as usize] > 0) {
                    return true
                }
            },
            Color::White if x > 0 => {
                if (y != 7 && (1u64 << (idx - 7)) & self.bitboard[Piece::Pawn as usize][Color::White as usize] > 0) 
                    || (y != 0 && (1u64 << (idx - 9)) & self.bitboard[Piece::Pawn as usize][Color::White as usize] > 0) {
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


    fn get_piece_char_and_color(&self, square_idx: usize) -> (char, &'static str) {
        let bit = 1u64 << square_idx;

        let is_white = self.occupied[Color::White as usize] & bit != 0;
        let is_black = self.occupied[Color::Black as usize] & bit != 0;

        if !is_white && !is_black {
            return (' ', "");
        }

        let color = if is_white { "\x1b[97m\x1b[1m" } else { "\x1b[30m\x1b[1m" };
        let piece = 
            if self.bitboard[Piece::Pawn as usize][Color::White as usize] & bit != 0 { '♟' }
            else if self.bitboard[Piece::Pawn as usize][Color::Black as usize] & bit != 0 { '♙' }
            else if self.bitboard[Piece::Knight as usize][Color::White as usize] & bit != 0 { '♞' }
            else if self.bitboard[Piece::Knight as usize][Color::Black as usize] & bit != 0 { '♘' }
            else if self.bitboard[Piece::Bishop as usize][Color::White as usize] & bit != 0 { '♝' }
            else if self.bitboard[Piece::Bishop as usize][Color::Black as usize] & bit != 0 { '♗' }
            else if self.bitboard[Piece::Rook as usize][Color::White as usize] & bit != 0 { '♜' }
            else if self.bitboard[Piece::Rook as usize][Color::Black as usize] & bit != 0 { '♖' }
            else if self.bitboard[Piece::Queen as usize][Color::White as usize] & bit != 0 { '♛' }
            else if self.bitboard[Piece::Queen as usize][Color::Black as usize] & bit != 0 { '♕' }
            else if self.bitboard[Piece::King as usize][Color::White as usize] & bit != 0 { '♚' }
            else { '♔' };

        (piece, color)
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for rank in (0..8).rev() {
            output.push_str(&format!("{} |", rank + 1));

            for file in 0..8 {
                let square_idx = rank * 8 + file;
                let is_light_square = (rank + file) % 2 != 0;
                
                let bg_color = if is_light_square { "\x1b[48;5;209m" } else { "\x1b[48;5;94m" };
                let (symbol, fg_color) = self.get_piece_char_and_color(square_idx);
                
                output.push_str(&format!("{}{}{} \x1b[0m", bg_color, fg_color, symbol));
            }
            output.push_str("|\n");
        }
        output.push_str("   a b c d e f g h");
        
        write!(f, "{}", output)
    }
}
