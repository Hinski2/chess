use super::super::board::Board;
use super::super::piece_move::{PieceMove, MoveFlag};
use super::super::Color;

impl Board {
    pub(in crate::board) fn generate_pawn_moves_white(&self, pawn_pos: u64) -> Vec<PieceMove> {
        // setup
        let empty_tiles = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let mut moves = Vec::with_capacity(16);
        let ep_mask = match self.board_state.en_passant {
            Some(idx) => 1u64 << idx,
            None => 0,
        };

        // single push
        let mut single_push = (pawn_pos << 8) & empty_tiles;
        while single_push != 0 {
            let to = single_push.trailing_zeros() as u8;
            let from = to - 8;

            if to >= 56 {
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToBishop });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToKnight });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToQueen });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToRook });
            } else {
                moves.push(PieceMove { from, to, flag: MoveFlag::Normal });
            }

            single_push &= single_push - 1; // pop lsb
        }

        // double push
        const RANK_2: u64 = 0x0000_0000_0000_ff00;
        let mut double_push = ((((pawn_pos & RANK_2) << 8) & empty_tiles) << 8) & empty_tiles;
        while double_push != 0 {
            let to = double_push.trailing_zeros() as u8;
            let from = to - 16;

            moves.push(PieceMove { from, to, flag: MoveFlag::DoublePawnPush });
            double_push &= double_push - 1; // pop lsb
        }

        // left captures
        const VALID_LEFT_CAPTURES_TILES: u64 = 0xfefe_fefe_fefe_fefe; // B - H 
        let mut left_attact = ((pawn_pos & VALID_LEFT_CAPTURES_TILES) << 7) & (self.occupied[Color::Black as usize] | ep_mask);
        while left_attact != 0 {
            let to = left_attact.trailing_zeros() as u8;
            let from = to - 7;

            if to >= 56 {
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToBishopAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToKnightAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToQueenAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToRookAndCapture });
            } else if to == self.board_state.en_passant.unwrap_or(0) {
                moves.push(PieceMove { from, to, flag: MoveFlag::EnPassantCapture }); 
            } else {
                moves.push(PieceMove { from, to, flag: MoveFlag::Capture });
            }

            left_attact &= left_attact - 1; // pop lsb
        }

        // right captures
        const VALID_RIGHT_CAPTURES_TILES: u64 = 0x7f7f_7f7f_7f7f_7f7f; // A - G 
        let mut right_attack = ((pawn_pos & VALID_RIGHT_CAPTURES_TILES) << 9) & (self.occupied[Color::Black as usize] | ep_mask);
        while right_attack != 0 { 
            let to = right_attack.trailing_zeros() as u8;
            let from = to - 9;

            if to >= 56 {
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToBishopAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToKnightAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToQueenAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToRookAndCapture });
            } else if to == self.board_state.en_passant.unwrap_or(0) {
                moves.push(PieceMove { from, to, flag: MoveFlag::EnPassantCapture }); 
            } else {
                moves.push(PieceMove { from, to, flag: MoveFlag::Capture });
            }

            right_attack &= right_attack - 1; // pop lsb
        }

        return moves;
    }

    pub(in crate::board) fn generate_pawn_moves_black(&self, pawn_pos: u64) -> Vec<PieceMove> {
        // setup
        let empty_tiles = !(self.occupied[Color::White as usize] | self.occupied[Color::Black as usize]);
        let mut moves = Vec::with_capacity(16);
        let ep_mask = match self.board_state.en_passant {
            Some(idx) => 1u64 << idx,
            None => 0,
        };

        // single push
        let mut single_push = (pawn_pos >> 8) & empty_tiles;
        while single_push != 0 {
            let to = single_push.trailing_zeros() as u8;
            let from = to + 8;

            if to < 8 {
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToBishop });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToKnight });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToQueen });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToRook });
            } else {
                moves.push(PieceMove { from, to, flag: MoveFlag::Normal });
            }

            single_push &= single_push - 1; // pop lsb
        }

        // double push
        const RANK_7: u64 = 0x00ff_0000_0000_0000;
        let mut double_push = ((((pawn_pos & RANK_7) >> 8) & empty_tiles) >> 8) & empty_tiles;
        while double_push != 0 {
            let to = double_push.trailing_zeros() as u8;
            let from = to + 16;

            moves.push(PieceMove { from, to, flag: MoveFlag::DoublePawnPush });
            double_push &= double_push - 1; // pop lsb
        }

        // left captures
        const VALID_LEFT_CAPTURES_TILES: u64 = 0xfefe_fefe_fefe_fefe; // B - H 
        let mut left_attact = ((pawn_pos & VALID_LEFT_CAPTURES_TILES) >> 9) & (self.occupied[Color::White as usize] | ep_mask);
        while left_attact != 0 {
            let to = left_attact.trailing_zeros() as u8;
            let from = to + 9;

            if to < 8 {
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToBishopAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToKnightAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToQueenAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToRookAndCapture });
            } else if to == self.board_state.en_passant.unwrap_or(0) {
                moves.push(PieceMove { from, to, flag: MoveFlag::EnPassantCapture }); 
            } else {
                moves.push(PieceMove { from, to, flag: MoveFlag::Capture });
            }

            left_attact &= left_attact - 1; // pop lsb
        }

        // right captures
        const VALID_RIGHT_CAPTURES_TILES: u64 = 0x7f7f_7f7f_7f7f_7f7f;// A - G 
        let mut right_attack = ((pawn_pos & VALID_RIGHT_CAPTURES_TILES) >> 7) & (self.occupied[Color::White as usize] | ep_mask);
        while right_attack != 0 { 
            let to = right_attack.trailing_zeros() as u8;
            let from = to + 7;

            if to < 8 {
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToBishopAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToKnightAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToQueenAndCapture });
                moves.push(PieceMove { from, to, flag: MoveFlag::PromoteToRookAndCapture });
            } else if to == self.board_state.en_passant.unwrap_or(0) {
                moves.push(PieceMove { from, to, flag: MoveFlag::EnPassantCapture }); 
            } else {
                moves.push(PieceMove { from, to, flag: MoveFlag::Capture });
            }

            right_attack &= right_attack - 1; // pop lsb
        }

        return moves;
    }

}
