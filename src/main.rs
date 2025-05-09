fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{r1}");
    println!("{r2}");
    
    let r3 = &mut s; // BIG PROBLEM

    println!("{r3}");
}