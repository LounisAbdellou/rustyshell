use super::{node::Node, tree::Tree};

pub struct Parsor {
    i: usize,
    j: usize,
    node: Node,
    arg: String,
}

impl Parsor {
    pub fn new() -> Self {
        Parsor {
            i: 0,
            j: 0,
            arg: String::new(),
            node: Node::new(),
        }
    }

    // fn handle_quotes(&mut self, input: &String, args: &Vec<String>) {
    //     let is_single = input.as_bytes()[self.i] as char == '\'';
    //
    //     for character in input.chars().skip(1) {
    //         if is_single && character == '\'' {
    //             break;
    //         } else if !is_single && character == '\"' {
    //             break;
    //         }
    //
    //         self.j += 1;
    //     }
    //
    //     self.i = self.j;
    // }

    // fn handle_encapsulation(&mut self, input: &String, current_node: &mut Node) {}

    fn handle_arg(&mut self) {
        self.i = self.j + 1;
        self.node.push_arg(self.arg.clone());
        self.arg.clear();
    }

    pub fn read(&mut self, input: &String) -> Tree {
        let mut tree = Tree::new();

        for character in input.chars() {
            match character {
                ' ' => self.handle_arg(),
                '\n' => {
                    self.handle_arg();
                    tree.get_mut_root().push_child(self.node.clone());
                }
                // '(' => self.handle_encapsulation(input, &mut node),
                // '\"' | '\'' => self.handle_quotes(input, &args),
                _ => self.arg.push(character),
            }

            self.j += 1
        }

        tree
    }
}
