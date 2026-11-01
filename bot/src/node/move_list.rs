use std::debug_assert;

use engine::{board::piece_move::PieceMove, game::game::Game};

use crate::move_order::move_order::IMoveOrder;

#[derive(Default)]
pub struct MoveList {
    valid: bool,
    moves: Vec<PieceMove>, 
}

impl MoveList {
    pub fn get_moves(&mut self) -> &mut Vec<PieceMove> {
        debug_assert!(self.valid);
        &mut self.moves
    }

    pub fn get_ith_move(&mut self, i: usize) -> &mut PieceMove {
        debug_assert!(self.valid);
        debug_assert!(i < self.moves.len());
        &mut self.moves[i]
    }

    pub fn invalidate(&mut self) {
        self.valid = false;
    }

    pub fn prepare_moves<O: IMoveOrder>(&mut self, move_order: &mut O, game: &mut Game, deep: usize) -> usize {
        if !self.valid {
            game.board.generate_all_moves_into(&mut self.moves);
            move_order.sort(game, deep, &mut self.moves);
            self.valid = true;
        }

        self.moves.len()
    }
}