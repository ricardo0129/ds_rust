use rand::Rng;
use std::mem::swap;

pub struct Node<T> {
    key: T,
    priority: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    subtree: i32,
    sum: i32,
    to_prop: i32,
}

impl<T: Ord + Default + Clone> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            key: key,
            priority: rand::thread_rng().gen_range(0..i32::MAX),
            left: None,
            subtree: 1,
            sum: 0,
            to_prop: 0,
            right: None,
        }
    }

    pub fn insert(&mut self, v: Node<T>) {
        if v.key == self.key {
            //Element already exists don't do anything
        } else if v.key < self.key {
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

        self.recalc();
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

    pub fn recalc(&mut self) {
        let mut subsize: i32 = 1;
        if let Some(left) = &self.left {
            subsize += left.subtree;
        }
        if let Some(right) = &self.right {
            subsize += right.subtree;
        }
        self.subtree = subsize;
    }

    pub fn inorder(&self, keys: &mut Vec<T>) {
        if let Some(child) = &self.left {
            assert!(self.priority < child.priority);
            assert!(self.key >= child.key);

            child.inorder(keys);
        }
        println!("{}", self.subtree);
        keys.push(self.key.clone());
        if let Some(child) = &self.right {
            assert!(self.priority < child.priority);
            assert!(self.key <= child.key);

            child.inorder(keys);
        }
    }

    fn first(&self) -> T {
        if let Some(left) = &self.left {
            return left.first();
        } else {
            return self.key.clone();
        }
    }

    fn last(&self) -> T {
        if let Some(right) = &self.right {
            return right.first();
        } else {
            return self.key.clone();
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
        if let Some(left) = &mut self.left {
            left.recalc();
        }
        if let Some(right) = &mut self.right {
            right.recalc();
        }
        self.recalc();
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
        if let Some(left) = &mut self.left {
            left.recalc();
        }
        if let Some(right) = &mut self.right {
            right.recalc();
        }
        self.recalc();
    }

    fn increase_priority(&mut self, key: &T) {
        if self.key == *key {
            self.priority = i32::MAX;
            let mut root: &mut Node<T> = self;
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
        } else if self.key > *key {
            if let Some(left) = &mut self.left {
                left.increase_priority(key);
            }
        } else {
            if let Some(right) = &mut self.right {
                right.increase_priority(key);
            }
        }
    }

    pub fn delete(&mut self, key: &T) -> bool {
        self.increase_priority(key);
        if self.key == *key {
            //The root needs to be removed
            return false;
        } else {
            self.delete_leaf(key);
        }
        return true;
    }

    fn delete_leaf(&mut self, key: &T) {
        if self.key > *key {
            if self.left.as_mut().unwrap().key == *key {
                let _root = self.left.take();
            } else {
                self.left.as_mut().unwrap().delete_leaf(key);
            }
        } else if self.key < *key {
            if self.right.as_mut().unwrap().key == *key {
                let _root = self.right.take();
            } else {
                self.right.as_mut().unwrap().delete_leaf(key);
            }
        }
    }

    pub fn split(&mut self, key: &T) -> (Node<T>, Node<T>) {
        let mut fake_node = Node::new(key.clone());
        fake_node.priority = i32::MIN;
        self.insert(fake_node);
        let left = *self.left.take().unwrap();
        let right = *self.right.take().unwrap();
        self.delete(key);
        (left, right)
    }

    pub fn merge(left: Node<T>, right: Node<T>) -> Node<T> {
        let mut fake_node = Node::new(T::default());
        fake_node.insert(left);
        fake_node.insert(right);
        fake_node.delete(&T::default());
        fake_node
    }

    pub fn contains(&self, key: &T) -> bool {
        if self.key == *key {
            return true;
        } else if self.key < *key {
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

    fn nth(&self, pos: i32) -> T {
        let mut left_subtree = 0;
        if let Some(left) = &self.left {
            left_subtree += left.subtree;
        }
        if left_subtree == pos {
            return self.key.clone();
        } else if left_subtree > pos {
            return self.left.as_ref().unwrap().nth(pos);
        }
        return self.right.as_ref().unwrap().nth(pos - left_subtree - 1);
    }
}

pub struct Treap<T> {
    root: Option<Node<T>>,
}

impl<T: Ord + Default + Clone> Treap<T> {
    pub fn new() -> Self {
        return Self { root: None };
    }

    pub fn insert(&mut self, key: T) {
        let new_node: Node<T> = Node::new(key);
        if self.root.is_some() {
            self.root.as_mut().unwrap().insert(new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    pub fn contains(&mut self, key: &T) -> bool {
        if self.root.is_some() {
            return self.root.as_ref().unwrap().contains(key);
        }
        return false;
    }

    pub fn delete(&mut self, key: &T) {
        if self.root.as_ref().unwrap().contains(key) {
            let is_deleted = self.root.as_mut().unwrap().delete(key);
            if !is_deleted {
                self.root = None;
            }
        }
    }

    pub fn keys(&mut self) -> Vec<T> {
        let mut keys: Vec<T> = vec![];
        self.root.as_ref().unwrap().inorder(&mut keys);
        return keys;
    }

    pub fn first(&mut self) -> Option<T> {
        if self.root.is_some() {
            return Some(self.root.as_ref().unwrap().first());
        }
        return None;
    }

    pub fn last(&mut self) -> Option<T> {
        if self.root.is_some() {
            return Some(self.root.as_ref().unwrap().last());
        }
        return None;
    }

    pub fn len(&self) -> usize {
        if self.root.is_some() {
            return self.root.as_ref().unwrap().subtree as usize;
        }
        0
    }

    pub fn nth(&self, pos: usize) -> Option<T> {
        if self.root.is_none() || self.len() <= pos {
            return None;
        }
        return Some(self.root.as_ref().unwrap().nth(pos as i32));
    }
}

pub fn test() {
    let mut tree: Treap<i32> = Treap::new();
    for i in 0..10000000 {
        tree.insert(i);
    }
}
