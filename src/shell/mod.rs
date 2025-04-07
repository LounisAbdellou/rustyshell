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

    pub fn receive(&self, n_bytes: usize, input: &String) {
        if n_bytes < 1 {
            process::exit(0);
        }

        println!("{input}");
    }
}
