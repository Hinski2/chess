use crate::{allocators::{allocator::IAllocator, list_stack_node::ListStackNode, node::INode}, MAX_BRANCH_FACTOR, MAX_DEEP};

struct ListStackAllocator {
    nodes: Vec<ListStackNode>,
}

impl ListStackAllocator {
    pub fn new() -> Self {
        let mut nodes = Vec::with_capacity(MAX_DEEP);

        for _ in 0..MAX_DEEP {
            nodes.push(ListStackNode::new(MAX_BRANCH_FACTOR));
        }

        Self { nodes }
    }
}

impl IAllocator for ListStackAllocator {
    type Node = ListStackNode;
    type Key = usize;   // lvl

    fn clean(&mut self) {
        for node in &mut self.nodes {
            node.clear();
        }
    }

    // returns node at the lvl=key with cleared movers
    fn get_node(&mut self, key: Self::Key) -> &mut Self::Node {
        if cfg!(debug_assertions) {
            assert!(key <  self.nodes.len());
        }

        let node = &mut self.nodes[key];
        node.clear();

        node
    }
}
