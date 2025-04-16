mod shell;

use std::io;

use shell::Shell;

fn rustyshell() {
    let mut shell = Shell::new();
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => shell.receive(n, &input),
            Err(err) => panic!("Error: {err}"),
        }
    }
}

fn main() {
    rustyshell();
}
