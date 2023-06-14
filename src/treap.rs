use rand::Rng;
use std::collections::{BTreeSet, HashSet};
use std::mem;
use std::mem::{swap, take};

pub struct Node {
    key: i32,
    priority: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    subtree: i32,
    sum: i32,
    toProp: i32,
}

impl Node {
    pub fn new(key: i32) -> Self {
        Self {
            key: key,
            priority: rand::thread_rng().gen_range(0..i32::MAX),
            left: None,
            subtree: 1,
            sum: key,
            toProp: 0,
            right: None,
        }
    }

    pub fn insert(&mut self, v: Node) {
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

    pub fn inorder(&self, depth: i32) {
        if let Some(child) = &self.left {
            assert!(self.priority < child.priority);
            assert!(self.key >= child.key);

            child.inorder(depth + 1);
        }
        //println!("{}", self.key);
        if let Some(child) = &self.right {
            assert!(self.priority < child.priority);
            assert!(self.key <= child.key);

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

    fn increase_priority(&mut self, key: i32) {
        if self.key == key {
            self.priority = i32::MAX;
            let mut iter = 20;
            let mut root: &mut Node = self;
            while root.left.is_some() || root.right.is_some() {
                let left = &root.left;
                let right = &root.right;
                if right.is_none()
                    || (left.is_some()
                        && left.as_ref().unwrap().priority < right.as_ref().unwrap().priority)
                {
                    root.right_rotate();
                    root = root.right.as_mut().unwrap();
                } else {
                    root.left_rotate();
                    root = root.left.as_mut().unwrap();
                }
            }
        } else if self.key > key {
            if let Some(left) = &mut self.left {
                left.increase_priority(key);
            }
        } else {
            if let Some(right) = &mut self.right {
                right.increase_priority(key);
            }
        }
    }

    pub fn delete(&mut self, key: i32) -> bool {
        self.increase_priority(key);
        if self.key == key {
            //The root needs to be removed
            return false;
        } else {
            self.delete_leaf(key);
        }
        return true;
    }

    fn delete_leaf(&mut self, key: i32) {
        if self.key > key {
            if self.left.as_mut().unwrap().key == key {
                let root = self.left.take();
            } else {
                self.left.as_mut().unwrap().delete_leaf(key);
            }
        } else if self.key < key {
            if self.right.as_mut().unwrap().key == key {
                let root = self.right.take();
            } else {
                self.right.as_mut().unwrap().delete_leaf(key);
            }
        }
    }

    pub fn split(&mut self, key: i32) -> (Node, Node) {
        let mut fake_node = Node::new(key);
        fake_node.priority = i32::MIN;
        self.insert(fake_node);
        let left = *self.left.take().unwrap();
        let right = *self.right.take().unwrap();
        self.delete(key);
        (left, right)
    }

    pub fn merge(left: Node, right: Node) -> Node {
        let mut fake_node = Node::new(i32::MIN);
        fake_node.insert(left);
        fake_node.insert(right);
        fake_node.delete(i32::MIN);
        fake_node
    }

    pub fn contains(&self, key: i32) -> bool {
        if self.key == key {
            return true;
        } else if self.key < key {
            if let Some(child) = &self.right {
                return child.contains(key);
            }
        } else {
            if let Some(child) = &self.left {
                return child.contains(key);
            }
        }
        return false;
    }
}
