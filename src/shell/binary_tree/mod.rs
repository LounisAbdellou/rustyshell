mod binary_node;

use binary_node::BinaryNode;

pub struct BinaryTree {
    root: BinaryNode,
    depth: u32,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree {
            root: BinaryNode::new(),
            depth: 0,
        }
    }

    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn get_root(&self) -> &BinaryNode {
        &self.root
    }

    pub fn get_mut_root(&mut self) -> &mut BinaryNode {
        &mut self.root
    }
}
