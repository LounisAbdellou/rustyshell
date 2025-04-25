use std::collections::VecDeque;

pub enum Separator {
    And,
    Or,
}

pub struct Command {
    args: VecDeque<String>,
    separator: Option<Separator>,
}

impl Command {
    pub fn get_args(&self) -> &VecDeque<String> {
        &self.args
    }
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

    fn handle_arg(&mut self, args: &mut VecDeque<String>, arg: &mut String) {
        self.i = self.j + 1;
        args.push_back(arg.clone());
        arg.clear();
    }

    pub fn read(&mut self, input: &String) -> VecDeque<Command> {
        let mut arg = String::new();
        let mut args: VecDeque<String> = VecDeque::new();
        let mut commands: VecDeque<Command> = VecDeque::new();

        for character in input.chars() {
            match character {
                ' ' => self.handle_arg(&mut args, &mut arg),
                '\n' => {
                    self.handle_arg(&mut args, &mut arg);
                    commands.push_back(Command {
                        args: args.clone(),
                        separator: None,
                    });
                    args.clear();
                }
                _ => arg.push(character),
            }

            self.j += 1
        }

        commands
    }
}
