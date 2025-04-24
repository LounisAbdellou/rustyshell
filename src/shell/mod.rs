mod node;
mod parsor;
mod tree;

use std::process;

use parsor::Parsor;

pub struct Shell {
    parsor: Parsor,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            parsor: Parsor::new(),
        }
    }

    pub fn receive(&mut self, n_bytes: usize, input: &String) {
        if n_bytes < 1 {
            process::exit(0);
        }

        let tree = self.parsor.read(input);
    }
}
