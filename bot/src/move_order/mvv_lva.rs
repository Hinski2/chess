use std::cmp::Reverse;

use engine::{board::{Piece, piece_move::{MoveFlag::{self, EnPassantCapture, PromoteToKnightAndCapture, PromoteToQueenAndCapture}, PieceMove}}, game::game::Game};

use crate::move_order::move_order::IMoveOrder;

/// [victim][attacker] in order [Pawn, Knight, Bishop, Rook, Quuen, King]
const MVV_LVA: [[i32; 6]; 6] = [
    [15, 14, 13, 12, 11, 10], 
    [25, 24, 23, 22, 21, 20],
    [35, 34, 33, 32, 31, 30],
    [45, 44, 43, 42, 41, 40],
    [55, 54, 53, 52, 51, 50],
    [ 0,  0,  0,  0,  0,  0],
];

const CAPTURE_BONUS: i32 = 1_000;
const QUEEN_PROMOTION: i32 = 2_000;
const OTHER_PROMOTION: i32 = -2_000;

#[derive(Default)]
pub struct MvvLva;

impl MvvLva {
    pub fn new() -> Self {
        Self
    }

    pub(super) fn piece_move_score(game: &Game, piece_move: &PieceMove) -> i32 {
        let mut score = 0;
        let victim = match piece_move.flag {
            EnPassantCapture => Some(Piece::Pawn),
            MoveFlag::Capture | MoveFlag::PromoteToBishopAndCapture | MoveFlag::PromoteToRookAndCapture
            | PromoteToKnightAndCapture | PromoteToQueenAndCapture => game.board.try_extract_piece_at(piece_move.to as usize),
            _ => None,
        };

        score += match victim {
            Some(victim) => {
                let attacker = game
                    .board
                    .extract_piece_at(piece_move.from as usize);
                CAPTURE_BONUS + MVV_LVA[victim as usize][attacker as usize]
            },
            None => 0,
        };

        score += match piece_move.flag {
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToQueenAndCapture => QUEEN_PROMOTION,
            MoveFlag::PromoteToBishop | MoveFlag::PromoteToKnight | MoveFlag::PromoteToRook 
            | MoveFlag::PromoteToBishopAndCapture | MoveFlag::PromoteToKnightAndCapture | MoveFlag::PromoteToRookAndCapture => OTHER_PROMOTION,
            _ => 0,
        };

        score
    }
}

impl IMoveOrder for MvvLva {
    fn sort(&mut self, game: &Game, _deep: usize, moves: &mut [PieceMove]) {
        moves.sort_unstable_by_key(|piece_move| Reverse(Self::piece_move_score(game, piece_move)));
    }
}