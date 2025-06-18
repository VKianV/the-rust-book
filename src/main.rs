fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = String::from(" it is kian");
    let s4 = s1 + &s2 + &s3; // note s1 has been moved here and can no longer be used

    println!("{}", s4);
}
