use engine::game::game::Game;
use crate::node::node::INode;

pub trait IAllocator<N: INode> {
    type NodeId: Copy;

    fn get_root_id(&mut self, game: &Game) -> Self::NodeId;
    fn get_ith_child_id(&mut self, parent_id: Self::NodeId, i: usize, game: &Game) -> Self::NodeId;
    fn get_node(&mut self, id: Self::NodeId) -> &mut N;
    fn new_search(&mut self) {}
}