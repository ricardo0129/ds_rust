pub mod treap;
use crate::treap::Treap;
fn main() {
    let mut tree = Treap::new();
    let n: i32 = 20;
    for i in 0..n {
        tree.insert(i);
        tree.insert(i);
    }
    for i in 0..n / 2 {
        tree.delete(i);
    }
}
