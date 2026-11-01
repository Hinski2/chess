use crate::move_order::move_order::IMoveOrder;

#[derive(Default, Clone, Debug)]
pub struct NoOrder;

impl NoOrder {
    pub fn new() -> Self {
        Self
    }
}

impl IMoveOrder for NoOrder {
    fn sort(&mut self, _game: &engine::game::game::Game, _deep: usize, _moves: &mut [engine::board::piece_move::PieceMove]) {}
}