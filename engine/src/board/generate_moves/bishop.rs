use super::super::board::Board;
use super::super::piece_move::{PieceMove, MoveFlag};
use super::super::Color;

impl Board {
    pub(crate) fn generate_bishop_moves_white(&self, mut bishop_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(16);
        let empty = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let enemy = self.occupied[Color::Black as usize];

        let shifts: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

        while bishop_pos != 0 {
            let from = bishop_pos.trailing_zeros() as u8;
            let x = (from / 8) as i32;
            let y = (from % 8) as i32;

            for shift in shifts {
                let mut _pos = (x + shift.0, y + shift.1);

                while 0 <= _pos.0 && _pos.0 < 8 && 0 <= _pos.1 && _pos.1 < 8 {
                    let to = (_pos.0 * 8 + _pos.1) as u8;

                    if (1 << to) & empty > 0 {
                        moves.push(PieceMove { from, to, flag: MoveFlag::Normal });
                    } else if (1 << to) & enemy > 0 {
                        moves.push(PieceMove { from, to, flag: MoveFlag::Capture });
                        break;
                    } else {
                        break;
                    }

                    _pos.0 += shift.0;
                    _pos.1 += shift.1;
                }
            } 

            bishop_pos &= bishop_pos - 1;
        }

        moves
    }

    pub(crate) fn generate_bishop_moves_black(&self, mut bishop_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(16);
        let empty = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let enemy = self.occupied[Color::White as usize];

        let shifts: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

        while bishop_pos != 0 {
            let from = bishop_pos.trailing_zeros() as u8;
            let x = (from / 8) as i32;
            let y = (from % 8) as i32;

            for shift in shifts {
                let mut _pos = (x + shift.0, y + shift.1);

                while 0 <= _pos.0 && _pos.0 < 8 && 0 <= _pos.1 && _pos.1 < 8 {
                    let to = (_pos.0 * 8 + _pos.1) as u8;

                    if (1 << to) & empty > 0 {
                        moves.push(PieceMove { from, to, flag: MoveFlag::Normal });
                    } else if (1 << to) & enemy > 0 {
                        moves.push(PieceMove { from, to, flag: MoveFlag::Capture });
                        break;
                    } else {
                        break;
                    }

                    _pos.0 += shift.0;
                    _pos.1 += shift.1;
                }
            } 

            bishop_pos &= bishop_pos - 1;
        }

        moves
    }

}