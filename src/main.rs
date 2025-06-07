struct Rectangle {
    width: u32,
    hight: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        hight: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.hight
}
