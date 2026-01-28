use std::mem;

use engine::board::piece_move::PieceMove;
use crate::allocators::node::INode;

pub struct ListStackNode {
    pub(crate) score: i32, 
    pub(crate) vis: i32,
    pub(crate) moves: Vec<PieceMove>,
}

impl ListStackNode {
    pub fn new(capacity: usize) -> Self {
        Self {
            score: 0, 
            vis: 0, 
            moves: Vec::with_capacity(capacity)
        }
    }
}

impl INode for ListStackNode {
    fn clear(&mut self) {
        self.score = 0;
        self.vis = 0;
        self.moves.clear();                
    }

    fn len(&self) -> usize {
        self.moves.len() 
    }

    fn is_empty(&self) -> bool {
        self.moves.is_empty()     
    }

    fn as_slice(&self) -> &[PieceMove] {
       &self.moves 
    }

    fn as_slice_mut(&mut self) -> &mut [PieceMove] {
       &mut self.moves 
    }

    fn set_moves(&mut self, moves: Vec<PieceMove>) {
        self.moves = moves;
    }

    fn take_moves(&mut self) -> Vec<PieceMove> {
        mem::take(&mut self.moves)
    }
}
