use super::{binary_node::BinaryNode, binary_tree::BinaryTree};

pub struct Parsor {
    i: usize,
    j: usize,
}

impl Parsor {
    pub fn new() -> Self {
        Parsor { i: 0, j: 0 }
    }

    pub fn read(&mut self, input: &String) -> BinaryTree {
        let mut arg = String::new();
        let mut args: Vec<String> = Vec::new();
        let mut binary_tree = BinaryTree::new();

        for character in input.chars() {
            match character {
                ' ' | '\n' => {
                    self.i = self.j + 1;
                    args.push(arg.clone());
                    arg.clear();
                }
                _ => arg.push(character),
            }
            self.j += 1
        }

        binary_tree
    }
}
