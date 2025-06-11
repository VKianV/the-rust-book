struct Node {
    current: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let node3 = Node {
        current: 3,
        next: None,
    };
    let node2 = Node {
        current: 2,
        next: Some(Box::new(node3)),
    };
    let node1 = Node {
        current: 1,
        next: Some(Box::new(node2)),
    };
}
