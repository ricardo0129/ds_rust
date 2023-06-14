use rand::Rng;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::f64::INFINITY;
use std::mem::swap;
use std::rc::Rc;

struct Node {
    key: i32,
    priority: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    subtree: i32,
    sum: i32,
    toProp: i32,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key: key,
            priority: rand::thread_rng().gen_range(0..1000000000),
            left: None,
            subtree: 1,
            sum: key,
            toProp: 0,
            right: None,
        }
    }

    fn insert(&mut self, v: Node) {
        if v.key < self.key {
            if let Some(child) = &mut self.left {
                child.insert(v);
            } else {
                self.left = Some(Box::from(v));
            }
        } else {
            if let Some(child) = &mut self.right {
                child.insert(v);
            } else {
                self.right = Some(Box::from(v));
            }
        }

        if let Some(right) = &self.right {
            if right.priority < self.priority {
                self.left_rotate();
            }
        }

        if let Some(left) = &self.left {
            if left.priority < self.priority {
                self.right_rotate();
            }
        }
    }

    fn inorder(&self, depth: i32) {
        if let Some(child) = &self.left {
            // assert!(self.priority < child.priority);
            // assert!(self.key >= child.key);

            child.inorder(depth + 1);
        }
        //println!("{}", depth);
        if let Some(child) = &self.right {
            // assert!(self.priority < child.priority);
            // assert!(self.key <= child.key);

            child.inorder(depth + 1);
        }
    }

    fn left_rotate(&mut self) {
        if let Some(mut right) = self.right.take() {
            let a = self.left.take();
            swap(self, &mut *right);
            let c = self.left.take();
            self.left = Some(right);
            if let Some(left) = &mut self.left {
                left.left = a;
                left.right = c;
            }
        }
    }

    fn right_rotate(&mut self) {
        if let Some(mut left) = self.left.take() {
            let c = self.right.take();
            swap(self, &mut *left);
            let b = self.right.take();
            self.right = Some(left);
            if let Some(right) = &mut self.right {
                right.left = b;
                right.right = c;
            }
        }
    }

    fn delete(&mut self, key: i32) {
        if self.key == key {
            self.priority = i32::MAX;
            let mut iter = 20;
            while self.left.is_some() || self.right.is_some() {
                let left = &self.left;
                let right = &self.right;
                if right.is_none()
                    || (left.is_some()
                        && left.as_ref().unwrap().priority < right.as_ref().unwrap().priority)
                {
                    self.left.as_mut().unwrap().right_rotate();
                } else {
                    self.right.as_mut().unwrap().left_rotate();
                }
                iter -= 1;
                if iter == 0 {
                    break;
                }
            }
        } else if self.key > key {
            if let Some(left) = &mut self.left {
                left.delete(key);
            }
        } else {
            if let Some(right) = &mut self.right {
                right.delete(key);
            }
        }
    }

    fn split(&mut self, left: i32) -> (Node, Node) {
        (Node::new(0), Node::new(0))
    }

    fn merge() {}
}

fn main() {
    let mut root: Node = Node::new(0);

    for i in 0..10 {
        let new_node = Node::new(i);
        root.insert(new_node);
    }
    root.inorder(0);
    println!("-----------");
    root.delete(0);
    //root.right_rotate();
    //root.inorder(0);
}
