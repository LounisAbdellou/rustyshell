use super::{node::Node, tree::Tree};

pub struct Parsor {
    i: usize,
    j: usize,
}

impl Parsor {
    pub fn new() -> Self {
        Parsor { i: 0, j: 0 }
    }

    fn handle_quotes(&mut self, input: &String, args: &Vec<String>) {
        let is_single = input.as_bytes()[self.i] as char == '\'';

        for character in input.chars().skip(1) {
            if is_single && character == '\'' {
                break;
            } else if !is_single && character == '\"' {
                break;
            }

            self.j += 1;
        }

        self.i = self.j;
    }

    pub fn read(&mut self, input: &String) -> Tree {
        let tree = Tree::new();
        let mut arg = String::new();

        for character in input.chars() {
            let mut node = Node::new();

            match character {
                ' ' | '\n' => {
                    self.i = self.j + 1;
                    node.push_arg(arg.clone());
                    arg.clear();
                }
                // '\"' | '\'' => self.handle_quotes(input, &args),
                _ => arg.push(character),
            }

            self.j += 1
        }

        tree
    }
}
