use engine::game::game::Game;
use crate::evaluator::IEvaluator;

use super::utils::random_walk;

pub struct MonteCarloEvaluator {
    itr: usize,
}

impl MonteCarloEvaluator {
    pub fn new(itr: usize) -> Self {
        Self { itr }
    }
}

impl IEvaluator for MonteCarloEvaluator {
    fn evaluate(&self, game: &Game) -> i32 {
        let mut score = 0;

        for _ in 0..self.itr {
            let mut game_simulation = Game::from(game);
            score += random_walk(&mut game_simulation);
        }

        score
    }
}