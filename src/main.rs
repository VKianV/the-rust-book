struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { a, b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
