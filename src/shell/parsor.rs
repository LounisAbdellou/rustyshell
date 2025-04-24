use std::collections::VecDeque;

pub enum Separator {
    And,
    Or,
}

pub struct Command {
    args: Vec<String>,
    separator: Option<Separator>,
}

pub struct Parsor {
    i: usize,
    j: usize,
}

impl Parsor {
    pub fn new() -> Self {
        Parsor { i: 0, j: 0 }
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

    fn handle_arg(&mut self, args: &mut VecDeque<String>, arg: &String) {
        self.i = self.j + 1;
        args.push_back(arg.clone());
        // deque.push_back(arg.clone());
        // self.node.push_arg(self.arg.clone());
        // self.arg.clear();
    }

    pub fn read(&mut self, input: &String) -> VecDeque<VecDeque<String>> {
        let mut arg = String::new();
        let mut args: VecDeque<String> = VecDeque::new();
        let mut deque: VecDeque<VecDeque<String>> = VecDeque::new();

        for character in input.chars() {
            match character {
                ' ' => self.handle_arg(&mut args, &arg),
                '\n' => {
                    self.handle_arg(&mut args, &arg);
                    // tree.get_mut_root().push_child(self.node.clone());
                }
                // '(' => self.handle_encapsulation(input, &mut node),
                // '\"' | '\'' => self.handle_quotes(input, &args),
                _ => arg.push(character),
            }

            self.j += 1
        }

        deque
    }
}
