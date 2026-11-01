use crate::node::move_list::MoveList;

use super::node::INode;

#[derive(Default)]
pub struct NodeMcts {
    move_list: MoveList,
    pub visits: u32,
    pub score: f32,
}

impl INode for NodeMcts {
    fn get_move_list(&mut self) -> &mut MoveList {
        &mut self.move_list 
    }
}