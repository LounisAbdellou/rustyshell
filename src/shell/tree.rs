use super::node::Node;

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: Node::new() }
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }

    pub fn get_mut_root(&mut self) -> &mut Node {
        &mut self.root
    }
}
