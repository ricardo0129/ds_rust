struct Node {
    x: i32,
    y: i32,
}

impl Node {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn build(points: Vec<(i32, i32)>) -> Node {
        if points.len() == 1 {
            return Node::new(points[0].0, points[0].1);
        }

        return Node::new(0, 0);
    }
}

fn main() {}
