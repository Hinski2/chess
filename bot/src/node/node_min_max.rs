use crate::node::move_list::MoveList;

use super::node::INode;

#[derive(Default)]
pub struct NodeMinMax {
    move_list: MoveList,
}

impl INode for NodeMinMax {
    fn get_move_list(&mut self) -> &mut MoveList {
        &mut self.move_list 
    }
}