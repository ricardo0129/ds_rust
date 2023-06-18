struct Node {
    x: i32,
    y: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x: x,
            y: y,
            left: None,
            right: None,
        }
    }

    fn build(x_sorted: Vec<(i32, i32)>, y_sorted: Vec<(i32, i32)>, depth: i32) -> Node {
        if x_sorted.len() == 1 {
            return Node::new(x_sorted[0].0, x_sorted[0].1);
        }
        let mut x_left: Vec<(i32, i32)> = vec![];
        let mut y_left: Vec<(i32, i32)> = vec![];
        let mut x_right: Vec<(i32, i32)> = vec![];
        let mut y_right: Vec<(i32, i32)> = vec![];
        if depth % 2 == 0 {
            //split by X median
            let median = x_sorted[x_sorted.len() / 2].0;
            for i in 0..x_sorted.len() {
                if i == x_sorted.len() / 2 {
                    continue;
                }
                if x_sorted[i].0 <= median {
                    x_left.push(x_sorted[i]);
                } else {
                    x_right.push(x_sorted[i]);
                }
                if y_sorted[i].0 <= median {
                    y_left.push(y_sorted[i]);
                } else {
                    y_right.push(y_sorted[i]);
                }
            }
        } else {
            //Split by Y median
            let median = y_sorted[y_sorted.len() / 2].1;
            for i in 0..y_sorted.len() {
                if i == y_sorted.len() / 2 {
                    continue;
                }
                if x_sorted[i].1 <= median {
                    x_left.push(x_sorted[i]);
                } else {
                    x_right.push(x_sorted[i]);
                }
                if y_sorted[i].1 <= median {
                    y_left.push(y_sorted[i]);
                } else {
                    y_right.push(y_sorted[i]);
                }
            }
        }
        let left: Node = Node::build(x_left, y_left, depth + 1);
        let right: Node = Node::build(x_right, y_right, depth + 1);
        let mut root = Node::new(
            y_sorted[y_sorted.len() / 2].0,
            y_sorted[y_sorted.len() / 2].1,
        );
        root.left = Some(Box::new(left));
        root.right = Some(Box::new(right));
        return root;
    }
}

fn main() {}
