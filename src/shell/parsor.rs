pub struct Parsor {
    command_line: Option<String>,
}

impl Parsor {
    pub fn new() -> Self {
        Parsor { command_line: None }
    }
}
