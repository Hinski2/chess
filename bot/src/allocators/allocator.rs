use crate::allocators::node::INode;

pub trait IAllocator {
    type Node: INode;
    type Key: Copy; 
    
    fn clean(&mut self);
    fn get_node(&mut self, key: Self::Key) -> &mut Self::Node;
}
