pub mod treap;
use crate::treap::Node;
fn main() {
    let mut root: Node = Node::new(0);
    for i in 0..1000000 {
        root.insert(Node::new(i));
        root.contains(0);
        root.contains(0);
        root.contains(0);
    }
}
