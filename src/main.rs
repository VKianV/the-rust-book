/*
 use the_book::{debug, find_largest};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = find_largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = find_largest(&char_list);
    println!("The largest char is {result}");
}
*/
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
