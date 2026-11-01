use std::debug_assert;

use engine::game::game::Game;

use crate::{allocators::allocator::IAllocator, node::node::INode};

pub struct ListStackAllocator<N: INode> {
    nodes: Vec<N>,
}

impl<N: INode> ListStackAllocator<N> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn fresh(&mut self, deep: usize) -> usize {
        if deep >= self.nodes.len() {
            self.nodes.resize_with(deep + 1, N::default);
        }

        self.nodes[deep].reset();
        deep
    }
}

impl<N: INode> IAllocator<N> for ListStackAllocator<N> {
    type NodeId = usize;

    fn get_root_id(&mut self, _game: &Game) -> Self::NodeId {
        self.fresh(0)
    }

    /// invoke only afler make a move
    fn get_ith_child_id(&mut self, parent_id: Self::NodeId, _i: usize, _game: &Game) -> Self::NodeId {
        self.fresh(parent_id + 1)
    }

    fn get_node(&mut self, id: Self::NodeId) -> &mut N {
        debug_assert!(id < self.nodes.len());
        &mut self.nodes[id]
    }
}
