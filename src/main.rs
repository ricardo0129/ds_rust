use std::cell::RefCell;
use std::cmp::Ordering;
use std::mem::swap;
use std::rc::Rc;

type Link = Option<Box<Node>>;

pub struct Node {
    key: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key: key,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, v: Node) {
        match v.key.cmp(&self.key) {
            Ordering::Equal => {
                //The Key ALREADY exists
            }
            Ordering::Less => {
                //The Key is LESS than root
                if let Some(left) = &mut self.left {
                    left.insert(v);
                } else {
                    self.left = Some(Box::new(v));
                }
            }
            Ordering::Greater => {
                //The Key is MORE than root
                if let Some(right) = &mut self.right {
                    right.insert(v);
                } else {
                    self.right = Some(Box::new(v));
                }
            }
        }
    }

    fn zig_zag(&mut self, left: bool) {
        if left {
            let mut w = self.left.take();
            if let Some(w) = &mut w {
                w.rotate_left();
            }
            self.left = w;
            self.rotate_right();
        } else {
            let mut w = self.right.take();
            if let Some(w) = &mut w {
                w.rotate_right();
            }
            self.right = w;
            self.rotate_left();
        }
    }

    fn zig_zig(&mut self, left: bool) {
        if left {
            let mut y = self.left.take();
            if let Some(y) = &mut y {
                y.rotate_right();
            }
            self.left = y;
            self.rotate_right();
        } else {
            let mut y = self.right.take();
            if let Some(y) = &mut y {
                y.rotate_left();
            }
            self.right = y;
            self.rotate_left();
        }
    }

    //      U             V
    //     / \           / \
    //    V  |C|  <=>  |A|  U
    //   / \               / \
    // |A| |B|           |B| |C|
    // right rotate =>
    // left  rotate <=
    fn rotate_right(&mut self) {
        if let Some(mut u) = self.left.take() {
            swap(self, &mut *u);
            let b = self.right.take();
            u.left = b;
            self.right = Some(u);
        }
    }

    fn rotate_left(&mut self) {
        if let Some(mut v) = self.right.take() {
            swap(self, &mut *v);
            let b = self.left.take();
            v.right = b;
            self.left = Some(v);
        }
    }

    fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        println!("{}", self.key);
        if let Some(right) = &self.right {
            right.inorder();
        }
    }
}

// mod treap;
// use crate::treap::test;
fn main() {
    let mut root = Node::new(4);
    let adding = vec![7, 2, 3];
    for a in adding {
        let new_node = Node::new(a);
        root.insert(new_node);
    }
    root.zig_zag(true);
    root.inorder();
}
