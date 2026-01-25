use super::{Color, Piece, PieceColor};
use super::board::Board;
use super::piece_move::{PieceMove};

impl Board {
    pub(super) fn handle_undo_move(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = 1u64 << piece_move.from;
            let to_bit = 1u64 << piece_move.to;

            assert!(self.occupied[us as usize] & to_bit > 0);
            assert!(self.occupied[them as usize] & to_bit == 0);
            assert!(self.occupied[us as usize] & from_bit == 0);
            assert!(self.occupied[them as usize] & from_bit == 0);

            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece() as usize;
            assert!(self.bitboard[piece_type_to][us as usize] & to_bit > 0);
        }

        let piece = self.pieces[piece_move.to as usize].extract_piece();
        self.toggle_piece(us, piece, piece_move.to);
        self.toggle_piece(us, piece, piece_move.from);
    }

    pub(super) fn handle_undo_capture(&mut self, piece_move: &PieceMove, captured_piece: &Piece) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = 1u64 << piece_move.from;
            let to_bit = 1u64 << piece_move.to;

            assert!(self.occupied[us as usize] & to_bit > 0);
            assert!(self.occupied[them as usize] & to_bit == 0);
            assert!(self.occupied[us as usize] & from_bit == 0);
            assert!(self.occupied[them as usize] & from_bit == 0);

            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece() as usize;
            assert!(self.bitboard[piece_type_to][us as usize] & to_bit > 0);
        }

        let our_piece = self.pieces[piece_move.to as usize].extract_piece();

        self.toggle_piece(us, our_piece, piece_move.to);
        self.toggle_piece(us, our_piece, piece_move.from);
        self.toggle_piece(them, *captured_piece, piece_move.to);
    }

    pub(super) fn handle_undo_en_passant_capture(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();
        let victim_idx = if us == Color::White { piece_move.to - 8 } else { piece_move.to + 8};

        if cfg!(debug_assertions) {
            let from_bit = 1u64 << piece_move.from;
            let to_bit  = 1u64 << piece_move.to;
            let victim_bit = 1u64 << victim_idx;

            assert!(self.occupied[us as usize] & to_bit > 0);
            assert!(self.occupied[them as usize] & to_bit == 0);
            assert!(self.occupied[us as usize] & from_bit == 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & victim_bit == 0);
            assert!(self.occupied[them as usize] & victim_bit == 0);

            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece();
            assert!(piece_type_to == Piece::Pawn);

            assert!(self.bitboard[Piece::Pawn as usize][us as usize] & to_bit > 0);
            assert!(self.bitboard[Piece::Pawn as usize][them as usize] & victim_bit == 0);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.to);
        self.toggle_piece(us, Piece::Pawn, piece_move.from);
        self.toggle_piece(them, Piece::Pawn, victim_idx);
    }

    pub(super) fn handle_undo_promotion(&mut self, piece_move: &PieceMove) {
        let to_bit = 1u64 << piece_move.to;
        let us = self.side_to_move;

        if cfg!(debug_assertions) {
            assert!(self.occupied[us as usize] & to_bit != 0);
            assert!(self.pieces[piece_move.to as usize] != PieceColor::None);

            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece();
            assert!(piece_type_to != Piece::Pawn);
            assert!(self.bitboard[piece_type_to as usize][us as usize] & to_bit > 0);
        }

        let piece_type_to = self.pieces[piece_move.to as usize].extract_piece();
        self.toggle_piece(us, piece_type_to, piece_move.to);
        self.toggle_piece(us, Piece::Pawn, piece_move.to);
    }

    pub(super) fn handle_undo_castle(&mut self, piece_move: &PieceMove) {
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

            assert!(self.occupied[us as usize] & to_bit > 0);
            assert!(self.occupied[us as usize] & from_bit == 0);

            assert!(self.occupied[us as usize] & (1u64 << rook_to) > 0);
            assert!(self.occupied[us as usize] & (1u64 << rook_from) == 0);

            assert!(self.pieces[piece_move.to as usize].extract_piece() == Piece::King);
        }

        self.toggle_piece(us, Piece::King, piece_move.to);
        self.toggle_piece(us, Piece::King, piece_move.from);
        self.toggle_piece(us, Piece::Rook, rook_to);
        self.toggle_piece(us, Piece::Rook, rook_from);
    }

    pub(super) fn handle_undo_double_pawn_push(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = 1u64 << piece_move.from;
            let to_bit = 1u64 << piece_move.to;

            assert!(self.occupied[us as usize] & to_bit > 0);
            assert!(self.occupied[them as usize] & to_bit == 0);
            assert!(self.occupied[us as usize] & from_bit == 0);
            assert!(self.occupied[them as usize] & from_bit == 0);

            assert!(self.pieces[piece_move.to as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.from as usize] == PieceColor::None);

            assert!(self.bitboard[Piece::Pawn as usize][us as usize] & to_bit > 0);
            assert!(self.pieces[piece_move.to as usize].extract_piece() == Piece::Pawn);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.to);
        self.toggle_piece(us, Piece::Pawn, piece_move.from);
    }
}