use super::node::Node;
use engine::board::piece_move::{PieceMove, MoveFlag};

pub(super) struct Memory {
    pub(super) data: Vec<Node>,
    pub(super) deep: usize,
}

impl Memory {
    pub(super) fn new(deep: usize) -> Memory {
        let mut size: usize = 0;
        for _ in 0..deep {
            size *= 64;
        }
        
        let data = (0..size)
            .map(|i| Node::new(i, PieceMove { from: 0, to: 0, flag: MoveFlag::None }))
            .collect::<Vec<Node>>();

        Memory { data: data, deep: deep }
    } 
}
