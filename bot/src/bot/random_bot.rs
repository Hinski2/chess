use engine::{board::piece_move::PieceMove, game::game::Game};
use rand::prelude::IndexedRandom;

use crate::bot::bot::IBot;

pub struct RandomBot;

impl RandomBot {
    pub fn new() -> Self {
        Self
    }
}

impl IBot for RandomBot {
    fn get_best_move(&mut self, game: &Game) -> PieceMove {
        let mut game = Game::from(game);

        let moves = game.board.generate_all_moves();
        moves.choose(&mut rand::rng()).unwrap().clone()
    }
}
