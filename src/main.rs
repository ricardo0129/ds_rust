use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;

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
        match key.cmp(&self.key) {
            Ordering::Equal => {
                //The Key ALREADY exists
            }
            Ordering::Less => {
                //The Key is LESS than root
                if let Some(left) = &mut self.left {
                    left.insert(key);
                } else {
                    let v = Box::new(Node::new(key));
                    self.add_left_child(v);
                    if let Some(l) = &mut self.left {
                        l.rebalance(true);
                    }
                }
            }
            Ordering::Greater => {
                //The Key is MORE than root
                if let Some(right) = &mut self.right {
                    right.insert(key);
                } else {
                    let v = Box::new(Node::new(key));
                    self.add_right_child(v);
                    if let Some(r) = &mut self.right {
                        r.rebalance(false);
                    }
                }
            }
        }
    }

    fn rebalance(&mut self, left: bool) {
        let mut iter = 5;
        let actual_key = self.key;
        let mut root = self;
        loop {
            assert!(root.key == actual_key);
            if root.parent().is_none() {
                break;
            }
            if let Some(p) = &mut root.parent() {
                let pk = p.key;
                println!("parent {}", pk);
                if let Some(g) = &mut p.parent() {
                    let b = g.right.is_none();
                    println!("grand {}", g.key);
                    let k = g.left.as_ref().unwrap().key;
                    if b || (k == pk) {
                        //grandparent -> left
                        if left {
                            println!("case 1");
                            //left -> left
                            g.zig_zig(false);
                            root = root.parent().unwrap();
                            root = root.parent().unwrap();
                        } else {
                            println!("case 2");
                            //left -> right
                            g.zig_zag(true);
                            root = root.parent().unwrap();
                        }
                    } else {
                        //grandparent -> right
                        if left {
                            //right -> left
                            println!("case 3");
                            g.zig_zag(false);
                            root = root.parent().unwrap();
                        } else {
                            //right -> right
                            println!("case 4");
                            g.zig_zig(true);
                            root = root.parent().unwrap();
                            root = root.parent().unwrap();
                        }
                    }
                } else {
                    //only one parent just rotate once
                    if left {
                        println!("case 5");
                        p.rotate_right();
                    } else {
                        println!("case 6");
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
            // left -> right
            let mut w = self.left.take();
            if let Some(w) = &mut w {
                w.rotate_left();
            }
            //self.left = w;
            self.add_left_child(w.unwrap());
            self.rotate_right();
        } else {
            // right -> left
            let mut w = self.right.take();
            if let Some(w) = &mut w {
                w.rotate_right();
            }
            //self.right = w;
            self.add_right_child(w.unwrap());
            self.rotate_left();
        }
    }

    fn zig_zig(&mut self, left: bool) {
        if left {
            let mut y = self.left.take();
            if let Some(y) = &mut y {
                y.rotate_right();
            }
            self.add_left_child(y.unwrap());
            self.rotate_right();
        } else {
            let mut y = self.right.take();
            if let Some(y) = &mut y {
                y.rotate_left();
            }
            self.add_right_child(y.unwrap());
            self.rotate_left();
        }
    }

    //      U             V0
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
            self.parent = p;
            let b = self.right.take();
            if b.is_some() {
                u.add_left_child(b.unwrap());
            }
            self.add_right_child(u);
        }
    }

    fn rotate_left(&mut self) {
        if let Some(mut v) = self.right.take() {
            let p = self.parent;
            swap(self, &mut *v);
            self.parent = p;
            let b = self.left.take();
            if b.is_some() {
                v.add_right_child(b.unwrap());
            }
            self.add_left_child(v);
        }
    }

    fn inorder(&mut self, depth: i32) {
        if let Some(left) = &mut self.left {
            left.inorder(depth + 1);
        }
        let k = self.key;
        println!("key{} depth{}", self.key, depth);
        if let Some(right) = &mut self.right {
            right.inorder(depth + 1);
        }
    }
}

// mod treap;
// use crate::treap::test;
fn main() {
    // let mut root = Node::new(4);
    // let adding = vec![7, 2, 3];
    // for a in adding {
    //     root.insert(a);
    // }
    let mut root = Node::new(1000);
    for i in 2..100 {
        let key = rand::thread_rng().gen_range(0..i32::MAX);
        println!("insert {}", key);
        root.insert(key);
    }
    root.inorder(0);
}
