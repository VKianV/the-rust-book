fn main() {
    let s = "‍";

    println!("with this charecters");
    for c in s.chars() {
        println!("{}", c);
    }
    println!("with this bytes");
    for c in s.bytes() {
        println!("{}", c);
    }
}
