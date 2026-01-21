use super::super::board::Board;
use super::super::piece_move::PieceMove;

impl Board {
    pub(in crate::board) fn generate_queen_moves_white(&self, queen_pos: u64) -> Vec<PieceMove> {
        let mut rook_moves = self.generate_rook_moves_white(queen_pos);
        let mut bishop_moves = self.generate_bishop_moves_white(queen_pos);
        rook_moves.append(&mut bishop_moves);

        rook_moves
    }

    pub(in crate::board) fn generate_queen_moves_black(&self, queen_pos: u64) -> Vec<PieceMove> {
        let mut rook_moves = self.generate_rook_moves_black(queen_pos);
        let mut bishop_moves = self.generate_bishop_moves_black(queen_pos);
        rook_moves.append(&mut bishop_moves);

        rook_moves
    }
}