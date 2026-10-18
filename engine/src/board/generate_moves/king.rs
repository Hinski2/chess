use super::super::board::Board;
use super::super::piece_move::{PieceMove, MoveFlag};
use super::super::Color;

pub(crate) const KING_ATTACK: [u64; 64] = [
	0x0000000000000302, 0x0000000000000705, 0x0000000000000e0a, 0x0000000000001c14, 0x0000000000003828, 0x0000000000007050, 0x000000000000e0a0, 0x000000000000c040, 
	0x0000000000030203, 0x0000000000070507, 0x00000000000e0a0e, 0x00000000001c141c, 0x0000000000382838, 0x0000000000705070, 0x0000000000e0a0e0, 0x0000000000c040c0, 
	0x0000000003020300, 0x0000000007050700, 0x000000000e0a0e00, 0x000000001c141c00, 0x0000000038283800, 0x0000000070507000, 0x00000000e0a0e000, 0x00000000c040c000, 
	0x0000000302030000, 0x0000000705070000, 0x0000000e0a0e0000, 0x0000001c141c0000, 0x0000003828380000, 0x0000007050700000, 0x000000e0a0e00000, 0x000000c040c00000, 
	0x0000030203000000, 0x0000070507000000, 0x00000e0a0e000000, 0x00001c141c000000, 0x0000382838000000, 0x0000705070000000, 0x0000e0a0e0000000, 0x0000c040c0000000, 
	0x0003020300000000, 0x0007050700000000, 0x000e0a0e00000000, 0x001c141c00000000, 0x0038283800000000, 0x0070507000000000, 0x00e0a0e000000000, 0x00c040c000000000, 
	0x0302030000000000, 0x0705070000000000, 0x0e0a0e0000000000, 0x1c141c0000000000, 0x3828380000000000, 0x7050700000000000, 0xe0a0e00000000000, 0xc040c00000000000, 
	0x0203000000000000, 0x0507000000000000, 0x0a0e000000000000, 0x141c000000000000, 0x2838000000000000, 0x5070000000000000, 0xa0e0000000000000, 0x40c0000000000000, 
];

const REQUIRED_EMPTY_FIELDS_FOR_WHITE_LEFT_CASTLE: u64 = (1u64 << 1) | (1u64 << 2) | (1u64 << 3);
const REQUIRED_EMPTY_FIELDS_FOR_WHITE_RIGHT_CASTLE: u64 = (1u64 << 5) | (1u64 << 6);
const REQUIRED_EMPTY_FIELDS_FOR_BLACK_LEFT_CASTLE: u64 = (1u64 << 57) | (1u64 << 58) | (1u64 << 59);
const REQUIRED_EMPTY_FIELDS_FOR_BLACK_RIGHT_CASTLE: u64 =(1u64 << 61) | (1u64 << 62);

impl Board {
    fn can_castle(&self, has_castle_right: bool, required_empty_tiles: u64) -> bool {
        let occupied_tiles: u64 = self.occupied[Color::White as usize] | self.occupied[Color::Black as usize];

        has_castle_right && required_empty_tiles & occupied_tiles == 0
    }

    pub(in crate::board) fn generate_king_moves_white(&self, king_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(8);
        let empty: u64 = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let enemy: u64 = self.occupied[Color::Black as usize];
        let idx = king_pos.trailing_zeros() as usize;

        // capture
        let mut attacks = KING_ATTACK[idx] & enemy;
        while attacks != 0 {
            let attack_idx = attacks.trailing_zeros() as u8;
            moves.push(PieceMove { from: idx as u8, to: attack_idx, flag: MoveFlag::Capture });

            attacks &= attacks - 1;
        }

        // not capture
        let mut attacks = KING_ATTACK[idx] & empty;
        while attacks != 0 {
            let attack_idx = attacks.trailing_zeros() as u8;
            moves.push(PieceMove { from: idx as u8, to: attack_idx, flag: MoveFlag::Normal });

            attacks &= attacks - 1;
        }

        // castle
        if self.can_castle(self.board_state.castle_rights_white_left(), REQUIRED_EMPTY_FIELDS_FOR_WHITE_LEFT_CASTLE) {
            moves.push(PieceMove { from: 4, to: 2, flag: MoveFlag::Castling });
        }

        if self.can_castle(self.board_state.castle_rights_white_right(), REQUIRED_EMPTY_FIELDS_FOR_WHITE_RIGHT_CASTLE) {
            moves.push(PieceMove { from: 4, to: 6, flag: MoveFlag::Castling });
        }

        moves
    }


    pub(in crate::board) fn generate_king_moves_black(&self, king_pos: u64) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(8);
        let empty: u64 = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let enemy: u64 = self.occupied[Color::White as usize];
        let idx = king_pos.trailing_zeros() as usize;

        // capture
        let mut attacks = KING_ATTACK[idx] & enemy;
        while attacks != 0 {
            let attack_idx = attacks.trailing_zeros() as u8;
            moves.push(PieceMove { from: idx as u8, to: attack_idx, flag: MoveFlag::Capture });

            attacks &= attacks - 1;
        }

        // not capture
        let mut attacks = KING_ATTACK[idx] & empty;
        while attacks != 0 {
            let attack_idx = attacks.trailing_zeros() as u8;
            moves.push(PieceMove { from: idx as u8, to: attack_idx, flag: MoveFlag::Normal });

            attacks &= attacks - 1;
        }

        // castle
        if self.can_castle(self.board_state.castle_rights_black_left(), REQUIRED_EMPTY_FIELDS_FOR_BLACK_LEFT_CASTLE) {
            moves.push(PieceMove { from: 60, to: 58, flag: MoveFlag::Castling });
        }

        if self.can_castle(self.board_state.castle_rights_black_right(), REQUIRED_EMPTY_FIELDS_FOR_BLACK_RIGHT_CASTLE) {
            moves.push(PieceMove { from: 60, to: 62, flag: MoveFlag::Castling });
        }

        moves
    }
}

