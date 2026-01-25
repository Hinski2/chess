use engine::{board::piece_move::PieceMove, game::game::Game};

pub trait IBot {
    fn get_best_move(&mut self, game: &Game) -> PieceMove;
}
