use engine::{Color, Piece};

use crate::game::{Game, GameEnum};

impl Game {
    pub(crate) fn check_for_insufficient_material(&self) -> bool {
        let white_pieces = self.board.occupied[Color::White as usize].count_ones(); 
        let black_pieces = self.board.occupied[Color::Black as usize].count_ones(); 
        let total = white_pieces + black_pieces;

        if total == 2 {
            return true;
        }

        if total == 3 {
            let big_pieces = self.board.bitboard[Piece::Pawn as usize][Color::White as usize] |
                            self.board.bitboard[Piece::Pawn as usize][Color::Black as usize] | 
                            self.board.bitboard[Piece::Rook as usize][Color::White as usize] | 
                            self.board.bitboard[Piece::Rook as usize][Color::Black as usize] | 
                            self.board.bitboard[Piece::Queen as usize][Color::White as usize] | 
                            self.board.bitboard[Piece::Queen as usize][Color::Black as usize];

            if big_pieces == 0 {
                return true;
            }
        }

        false
    }

    // check for draws
    pub(crate) fn check_for_draws(&mut self, hsh: u64, new_half_move_clock: usize) {
        if new_half_move_clock >= 100 {
            self.game = GameEnum::TieBy50Rule;
            return;
        }

        if self.hshs[&hsh] >= 3 {
            self.game = GameEnum::TieByThreefoldRepetition;
            return;
        }

        if self.check_for_insufficient_material() {
            self.game = GameEnum::TieByInsufficientMaterial;
            return;
        }
    }

    // invoke it only if there is no possible moves to make
    pub(crate) fn check_for_mate_or_stalemate(&mut self) {
        if self.board.is_checked() {
            self.game = match self.board.get_size_to_move() {
                Color::White => GameEnum::BlackWon,
                Color::Black => GameEnum::WhiteWon,
            }
        } else {
            self.game = GameEnum::TieBySalemate;
        }
    }

}