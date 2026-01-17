use crate::{Color, Piece, PieceColor};
use crate::board::Board;
use crate::piece_move::{MoveFlag, PieceMove};

const CASTLING_RIGHTS_UPDATE: [u8; 64] = [
    0b1101, 0b1111, 0b1111, 0b1111, 0b1100, 0b1111, 0b1111, 0b1110,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b0111, 0b1111, 0b1111, 0b1111, 0b0011, 0b1111, 0b1111, 0b1011,
];

impl Board {
    pub(crate) fn handle_move(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit = (1 << piece_move.to) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit == 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] == PieceColor::None);

            let piece_type_from = self.pieces[piece_move.from as usize].extract_piece() as usize;
            assert!(self.bitboard[piece_type_from][us as usize] & from_bit > 0);
        }

        let piece = self.pieces[piece_move.from as usize].extract_piece();
        self.toggle_piece(us, piece, piece_move.from);
        self.toggle_piece(us, piece, piece_move.to);

        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.from as usize];
    }

    pub(crate) fn handle_capture(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit  = (1 << piece_move.to) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit > 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] != PieceColor::None);

            let piece_type_from = self.pieces[piece_move.from as usize].extract_piece() as usize;
            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece() as usize;
            assert!(self.bitboard[piece_type_from][us as usize] & from_bit > 0);
            assert!(self.bitboard[piece_type_to][them as usize] & to_bit > 0);
        }

        let our_piece = self.pieces[piece_move.from as usize].extract_piece();
        let their_piece = self.pieces[piece_move.to as usize].extract_piece();

        self.toggle_piece(us, our_piece, piece_move.from);
        self.toggle_piece(them, their_piece, piece_move.to);
        self.toggle_piece(us, our_piece, piece_move.to);

        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.from as usize];
        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.to as usize];
    }

    pub(crate) fn handle_en_passant_capture(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();
        let victim_idx = if us == Color::White { piece_move.to - 8 } else { piece_move.to + 8};

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit  = (1 << piece_move.to) as u64;
            let victim_bit = (1 << victim_idx) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit == 0);
            assert!(self.occupied[us as usize] & victim_bit == 0);
            assert!(self.occupied[them as usize] & victim_bit > 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] == PieceColor::None);
            assert!(self.pieces[victim_idx as usize] != PieceColor::None);

            let piece_type_from = self.pieces[piece_move.from as usize].extract_piece();
            let piece_type_victim = self.pieces[victim_idx as usize].extract_piece();
            assert!(piece_type_from == Piece::Pawn);
            assert!(piece_type_victim == Piece::Pawn);

            assert!(self.bitboard[Piece::Pawn as usize][us as usize] & from_bit > 0);
            assert!(self.bitboard[Piece::Pawn as usize][them as usize] & victim_bit > 0);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.from);
        self.toggle_piece(them, Piece::Pawn, victim_idx);
        self.toggle_piece(us, Piece::Pawn, piece_move.to);
    }

    // updates piece and the .to
    pub(crate) fn handle_promotion(&mut self, piece_move: &PieceMove) {
        let to_bit = (1 << piece_move.to) as u64;
        let us = self.side_to_move;

        if cfg!(debug_assertions) {
            assert!(self.occupied[us as usize] & to_bit != 0);
            assert!(self.pieces[piece_move.to as usize] != PieceColor::None);

            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece();
            assert!(piece_type_to == Piece::Pawn);
            assert!(self.bitboard[piece_type_to as usize][us as usize] & to_bit > 0);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.to);
        match piece_move.flag {
            MoveFlag::PromoteToKnight | MoveFlag::PromoteToKnightAndCapture => self.toggle_piece(us, Piece::Knight, piece_move.to), 
            MoveFlag::PromoteToBishop | MoveFlag::PromoteToBishopAndCapture => self.toggle_piece(us, Piece::Bishop, piece_move.to), 
            MoveFlag::PromoteToRook | MoveFlag::PromoteToRookAndCapture => self.toggle_piece(us, Piece::Rook, piece_move.to), 
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToQueenAndCapture => self.toggle_piece(us, Piece::Queen, piece_move.to), 
            _ => unreachable!(),
        }
    }

    pub(crate) fn handle_castle(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let (rook_from, rook_to) = match piece_move.to {
            6 => (7, 5), // white short
            2 => (0, 3), // white long
            62 => (63, 61), // black short
            58 => (56, 59), // black long
            _ => unreachable!("Error in handle_castle: innapropriate castle setup"),
        };

        if cfg!(debug_assertions) {
            let from_bit = 1u64 << piece_move.from;
            let to_bit = 1u64 << piece_move.to;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[us as usize] & to_bit == 0);

            assert!(self.occupied[us as usize] & (1u64 << rook_from) > 0);
            assert!(self.occupied[us as usize] & (1u64 << rook_to) == 0);

            assert!(self.pieces[piece_move.from as usize].extract_piece() == Piece::King);
        }

        self.toggle_piece(us, Piece::King, piece_move.from);
        self.toggle_piece(us, Piece::King, piece_move.to);
        self.toggle_piece(us, Piece::Rook, rook_from);
        self.toggle_piece(us, Piece::Rook, rook_to);

        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.from as usize];
        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.to as usize];
    }

    pub(crate) fn handle_double_pawn_push(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit = (1 << piece_move.to) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit == 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] == PieceColor::None);

            assert!(self.bitboard[Piece::Pawn as usize][us as usize] & from_bit > 0);
            assert!(self.pieces[piece_move.from as usize].extract_piece() == Piece::Pawn);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.from);
        self.toggle_piece(us, Piece::Pawn, piece_move.to);

        self.board_state.en_passant = Some(
            if us == Color::White { piece_move.from + 8 } else { piece_move.from - 8 }
        );
    }

    pub(crate) fn toggle_piece(&mut self, color: Color, piece: Piece, idx: u8) {
        let bit = 1u64 << idx;
        self.occupied[color as usize] ^= bit;
        self.bitboard[piece as usize][color as usize] ^= bit;

        if self.occupied[color as usize] & bit > 0 {
            self.pieces[idx as usize] = PieceColor::new(piece, color);
        } else {
            self.pieces[idx as usize] = PieceColor::None;
        }
    }
}