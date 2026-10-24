use super::super::board::Board;
use super::super::piece_move::PieceMove;

impl Board {
    pub(in crate::board) fn generate_queen_moves_white(&self, queen_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(16);
        self.generate_queen_moves_white_into(queen_pos, &mut moves);
        moves
    }

    pub(in crate::board) fn generate_queen_moves_black(&self, queen_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(16);
        self.generate_queen_moves_black_into(queen_pos, &mut moves);
        moves
    }

    pub(in crate::board) fn generate_queen_moves_white_into(&self, queen_pos: u64, out: &mut Vec<PieceMove>) {
        self.generate_rook_moves_white_into(queen_pos, out);
        self.generate_bishop_moves_white_into(queen_pos, out);
    }

    pub(in crate::board) fn generate_queen_moves_black_into(&self, queen_pos: u64, out: &mut Vec<PieceMove>) {
        self.generate_rook_moves_black_into(queen_pos, out);
        self.generate_bishop_moves_black_into(queen_pos, out);
    }
}