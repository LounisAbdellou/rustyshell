pub struct BinaryNode {
    kind: Option<String>,
    args: Option<Vec<String>>,
    left: Box<Option<BinaryNode>>,
    right: Box<Option<BinaryNode>>,
}

impl BinaryNode {
    pub fn new() -> Self {
        BinaryNode {
            kind: None,
            args: None,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    pub fn from(kind: String, args: Vec<String>) -> Self {
        BinaryNode {
            kind: Some(kind),
            args: Some(args),
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    pub fn get_kind(&self) -> &Option<String> {
        &self.kind
    }

    pub fn get_args(&self) -> &Option<Vec<String>> {
        &self.args
    }

    pub fn get_left(&self) -> &Option<BinaryNode> {
        &self.left
    }

    pub fn get_right(&self) -> &Option<BinaryNode> {
        &self.right
    }

    pub fn set_kind(&mut self, kind: String) {
        self.kind = Some(kind);
    }

    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = Some(args);
    }

    pub fn set_left(&mut self, node: BinaryNode) {
        self.left = Box::new(Some(node));
    }

    pub fn set_right(&mut self, node: BinaryNode) {
        self.right = Box::new(Some(node));
    }
}
