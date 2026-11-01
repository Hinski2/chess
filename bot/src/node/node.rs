use crate::node::move_list::MoveList;

pub trait INode: Default {
    fn get_move_list(&mut self) -> &mut MoveList;

    fn reset(&mut self) {
        self.get_move_list().invalidate();
    }
}