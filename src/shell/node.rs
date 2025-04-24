#[derive(Clone)]
pub enum NodeKind {
    Root,
    And,
    Or,
}

#[derive(Clone)]
pub struct Node {
    kind: NodeKind,
    args: Vec<String>,
    childs: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            kind: NodeKind::Root,
            args: Vec::new(),
            childs: Vec::new(),
        }
    }

    pub fn get_kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_childs(&self) -> &Vec<Node> {
        &self.childs
    }

    pub fn set_kind(&mut self, kind: NodeKind) {
        self.kind = kind;
    }

    pub fn push_arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    pub fn push_child(&mut self, child: Node) {
        self.childs.push(child);
    }
}
