use engine::board::piece_move::PieceMove;
use crate::node::move_list::MoveList;

use super::node::INode;

#[derive(Default)]
pub struct NodeAlfaBeta {
    move_list: MoveList,
    pub best_move: Option<PieceMove>,
}


impl INode for NodeAlfaBeta {
    fn get_move_list(&mut self) -> &mut MoveList {
        &mut self.move_list 
    }
}