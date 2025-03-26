use std::env;
use std::io;

fn set_env_variable(key: &str, value: &str) {
    env::set_var(key, value);
}

fn check_env() {
    match env::var("PWD") {
        Ok(_) => 0,
        _ => {
            if let Ok(path) = env::current_dir() {
                set_env_variable("PWD", path.to_str().unwrap());
            }

            1
        }
    };

    match env::var("_") {
        Ok(_) => 0,
        _ => {
            set_env_variable("_", "/usr/bin/env");
            1
        }
    };

    match env::var("SHLVL") {
        Ok(lvl) => {
            let next_lvl = lvl.parse::<i32>().unwrap() + 1;
            set_env_variable("SHLVL", next_lvl.to_string().as_str());
        }
        _ => {
            set_env_variable("SHLVL", "1");
        }
    };
}

fn rustyshell() {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => print!("{} => {}", n, input),
            Err(err) => print!("{}", err),
        }
    }
}

fn main() {
    check_env();
    rustyshell();
}
