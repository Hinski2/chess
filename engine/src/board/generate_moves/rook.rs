use super::super::board::Board;
use super::super::piece_move::{PieceMove, MoveFlag};
use super::super::Color;

impl Board {
    pub(in crate::board) fn generate_rook_moves_white(&self, mut rook_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(16);
        let empty = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let enemy = self.occupied[Color::Black as usize];

        while rook_pos != 0 {
            let idx = rook_pos.trailing_zeros() as u8;
            let (x, y) = (idx / 8, idx % 8);

            // left
            for j in (0..y).rev() {
                let _idx = x * 8 + j;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            // right 
            for j in y + 1..8 {
                let _idx = x * 8 + j;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            // up 
            for i in (0..x).rev() {
                let _idx = i * 8 + y;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            // down
            for i in x + 1..8 {
                let _idx = i * 8 + y;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            rook_pos &= rook_pos - 1;
        }

        moves
    }

    pub(in crate::board) fn generate_rook_moves_black(&self, mut rook_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(16);
        let empty = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let enemy = self.occupied[Color::White as usize];

        while rook_pos != 0 {
            let idx = rook_pos.trailing_zeros() as u8;
            let (x, y) = (idx / 8, idx % 8);

            // left
            for j in (0..y).rev() {
                let _idx = x * 8 + j;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            // right 
            for j in y + 1..8 {
                let _idx = x * 8 + j;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            // up 
            for i in (0..x).rev() {
                let _idx = i * 8 + y;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            // down
            for i in x + 1..8 {
                let _idx = i * 8 + y;
                if empty & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Normal })
                } else if enemy & (1 << _idx) > 0 {
                    moves.push(PieceMove { from: idx, to: _idx, flag: MoveFlag::Capture });
                    break;
                } else {
                    break;
                }
            }

            rook_pos &= rook_pos - 1;
        }

        moves
    }
}