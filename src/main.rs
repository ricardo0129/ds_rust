struct Node {
    key: i32,
    priority: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key: key,
            priority: 0,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let root: Node = Node::new(12);
}
