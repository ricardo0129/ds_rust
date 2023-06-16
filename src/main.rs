use rand::Rng;
use std::cmp::Ordering;
use std::mem::swap;
type Link = Option<Box<Node>>;

pub struct Node {
    key: i32,
    left: Link,
    right: Link,
    parent: Option<*mut Node>,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key: key,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn add_left_child(&mut self, mut child: Box<Node>) {
        child.parent = Some(self as *mut Node);
        self.left = Some(child);
    }

    fn add_right_child(&mut self, mut child: Box<Node>) {
        child.parent = Some(self as *mut Node);
        self.right = Some(child);
    }

    fn insert(&mut self, key: i32) {
        let mut root = self;
        loop {
            match key.cmp(&root.key) {
                Ordering::Equal => {
                    //The Key ALREADY exists
                    break;
                }
                Ordering::Less => {
                    //The Key is LESS than root
                    if root.left.is_some() {
                        //left.insert(key);
                        root = root.left.as_mut().unwrap();
                    } else {
                        let v = Box::new(Node::new(key));
                        root.add_left_child(v);
                        if let Some(l) = &mut root.left {
                            l.splay();
                        }
                        break;
                    }
                }
                Ordering::Greater => {
                    //The Key is MORE than root
                    if root.right.is_some() {
                        root = root.right.as_mut().unwrap();
                    } else {
                        let v = Box::new(Node::new(key));
                        root.add_right_child(v);
                        if let Some(r) = &mut root.right {
                            r.splay();
                        }
                        break;
                    }
                }
            }
        }
    }

    fn splay(&mut self) {
        let actual_key = self.key;
        let mut root = self;
        loop {
            assert!(root.key == actual_key);
            if root.parent().is_none() {
                break;
            }
            if let Some(p) = &mut root.parent() {
                let pk = p.key;
                let left: bool;
                if p.right.is_none() || (p.right.as_ref().unwrap().key != actual_key) {
                    left = true;
                } else {
                    left = false;
                }
                if let Some(g) = &mut p.parent() {
                    if g.right.is_none() || (g.right.as_ref().unwrap().key != pk) {
                        //grandparent -> left
                        if left {
                            //left -> left
                            g.zig_zig(true);
                            root = root.parent().unwrap().parent().unwrap();
                        } else {
                            //left -> right
                            g.zig_zag(true);
                            root = root.parent().unwrap();
                        }
                    } else {
                        //grandparent -> right
                        if left {
                            //right -> left
                            g.zig_zag(false);
                            root = root.parent().unwrap();
                        } else {
                            //right -> right
                            g.zig_zig(false);
                            root = root.parent().unwrap().parent().unwrap();
                        }
                    }
                } else {
                    //only one parent just rotate once
                    if left {
                        p.rotate_right();
                    } else {
                        p.rotate_left();
                    }
                    root = root.parent().unwrap();
                }
            }
        }
    }

    fn parent(&mut self) -> Option<&mut Node> {
        self.parent.map(|parent| unsafe { &mut *parent })
    }

    fn zig_zag(&mut self, left: bool) {
        if left {
            // left right
            let mut w = self.left.take();
            if let Some(w) = &mut w {
                w.rotate_left();
            }
            self.add_left_child(w.unwrap());
            self.rotate_right();
        } else {
            // right left
            let mut w = self.right.take();
            if let Some(w) = &mut w {
                w.rotate_right();
            }
            self.add_right_child(w.unwrap());
            self.rotate_left();
        }
    }

    fn zig_zig(&mut self, left: bool) {
        if left {
            //left left
            let mut y = self.left.take();
            if let Some(y) = &mut y {
                y.rotate_right();
            }
            self.add_left_child(y.unwrap());
            self.rotate_right();
            let mut z = self.right.take();
            if let Some(z) = &mut z {
                z.rotate_right();
            }
            self.add_right_child(z.unwrap());
        } else {
            //right right
            let mut y = self.right.take();
            if let Some(y) = &mut y {
                y.rotate_left();
            }
            self.add_right_child(y.unwrap());
            self.rotate_left();
            let mut z = self.left.take();
            if let Some(z) = &mut z {
                z.rotate_left();
            }
            self.add_left_child(z.unwrap());
        }
    }
    //      P             P
    //      |             |
    //      U             V
    //     / \           / \
    //    V  |C|  <=>  |A|  U
    //   / \               / \
    // |A| |B|           |B| |C|
    // right rotate =>
    // left  rotate <=
    fn rotate_right(&mut self) {
        if let Some(mut u) = self.left.take() {
            let p = self.parent;
            swap(self, &mut *u);
            let b = self.right.take();
            let a = self.left.take();
            if a.is_some() {
                self.add_left_child(a.unwrap());
            }
            if b.is_some() {
                u.add_left_child(b.unwrap());
            }
            let c = u.right.take();
            if c.is_some() {
                u.add_right_child(c.unwrap());
            }
            self.add_right_child(u);
            self.parent = p;
        }
    }

    fn rotate_left(&mut self) {
        if let Some(mut v) = self.right.take() {
            let p = self.parent;
            swap(self, &mut *v);
            self.parent = p;
            let b = self.left.take();
            let c = self.right.take();
            if c.is_some() {
                self.add_right_child(c.unwrap());
            }
            if b.is_some() {
                v.add_right_child(b.unwrap());
            }
            let a = v.left.take();
            if a.is_some() {
                v.add_left_child(a.unwrap());
            }
            self.add_left_child(v);
            self.parent = p;
        }
    }

    fn inorder(&mut self, depth: i32) {
        if let Some(left) = &mut self.left {
            left.inorder(depth + 1);
        }
        let k = self.key;
        println!("key{} depth{}", self.key, depth);
        if let Some(p) = self.parent() {
            println!("    key {} parent {}", k, p.key);
        }
        if let Some(right) = &mut self.right {
            right.inorder(depth + 1);
        }
    }
}

mod treap;
use crate::treap::test;
use std::collections::{BTreeSet, HashSet};
fn main() {
    let mut root = Node::new(0);

    for i in 1..10000000 {
        let key = rand::thread_rng().gen_range(0..i32::MAX);
        root.insert(key);
    }
}
