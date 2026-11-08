use std::{cmp::Reverse, matches};

use engine::{board::piece_move::{MoveFlag, PieceMove}, game::game::Game};
use crate::move_order::{move_order::IMoveOrder, mvv_lva::MvvLva};

const FIRST_KILLER_SCORE: i32 = 900;
const SECOND_KILLER_SCORE: i32 = 800;

#[derive(Default)]
pub struct MvvLvaKillers {
    killers: Vec<[Option<PieceMove>; 2]>,
}

impl MvvLvaKillers {
    pub fn new() -> Self {
        Self { killers: Vec::new() }
    }

    fn piece_move_score(game: &Game, piece_move: &PieceMove, killers: &[Option<PieceMove>; 2]) -> i32 {
        let mut score = MvvLva::piece_move_score(game, piece_move);

        if matches!(piece_move.flag, MoveFlag::Normal | MoveFlag::DoublePawnPush | MoveFlag::Castling) {
            if killers[0].as_ref() == Some(piece_move) {
                score += FIRST_KILLER_SCORE;
            } else if killers[1].as_ref() == Some(piece_move) {
                score += SECOND_KILLER_SCORE;
            }
        }

        score
    }

    fn get_killers_at(&mut self, deep: usize) -> &mut [Option<PieceMove>; 2] {
        if deep >= self.killers.len() {
            self.killers.resize_with(deep + 1, | | [None, None]);
        }

        &mut self.killers[deep]
    }
}

impl IMoveOrder for MvvLvaKillers {
    fn sort(&mut self, game: &Game, deep: usize, moves: &mut [PieceMove]) {
        let killers = self.get_killers_at(deep);
        moves.sort_unstable_by_key(|piece_move| Reverse(Self::piece_move_score(game, piece_move, killers))); 
    }

    fn on_cutoff(&mut self, deep: usize, piece_move: &PieceMove) {
        if !matches!(piece_move.flag, MoveFlag::Normal | MoveFlag::DoublePawnPush | MoveFlag::Castling) {
            return;
        }

        let killers = self.get_killers_at(deep);
        if killers[0].as_ref() == Some(piece_move) {
            return
        }

        killers[1] = killers[0].take();
        killers[0] = Some(piece_move.clone());
    }

    fn new_search(&mut self) {
        self.killers.clear();
    }
}