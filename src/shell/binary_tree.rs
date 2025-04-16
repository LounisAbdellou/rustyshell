use super::binary_node::BinaryNode;

pub struct BinaryTree {
    root: BinaryNode,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree {
            root: BinaryNode::new(),
        }
    }

    pub fn get_root(&self) -> &BinaryNode {
        &self.root
    }

    pub fn get_mut_root(&mut self) -> &mut BinaryNode {
        &mut self.root
    }
}
