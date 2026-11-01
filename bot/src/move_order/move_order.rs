use engine::{board::piece_move::PieceMove, game::game::Game};


pub trait IMoveOrder {
    fn sort(&mut self, game: &Game, deep: usize, moves: &mut [PieceMove]);
    fn on_cutoff(&mut self, _deep: usize, _piece_move: PieceMove) {}
    fn new_search(&mut self) {}
}