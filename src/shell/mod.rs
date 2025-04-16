mod binary_node;
mod binary_tree;
mod parsor;

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

        let binary_tree = self.parsor.read(input);
    }
}
