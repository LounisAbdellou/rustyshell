enum NodeKind {
    Root,
    And,
    Or,
}

pub struct Node {
    kind: Option<NodeKind>,
    args: Vec<String>,
    childs: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            kind: None,
            args: Vec::new(),
            childs: Vec::new(),
        }
    }

    pub fn get_kind(&self) -> &Option<NodeKind> {
        &self.kind
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_childs(&self) -> &Vec<Node> {
        &self.childs
    }

    pub fn set_kind(&mut self, kind: NodeKind) {
        self.kind = Some(kind);
    }

    pub fn push_arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    pub fn push_child(&mut self, child: Node) {
        self.childs.push(child);
    }
}
