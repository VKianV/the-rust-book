fn main() {
    let v0: Vec<i32> = vec![1, 2, 3];

    let v1 = v0.iter();
    let v2 = v0.iter().map(|x| x + 1);
    let v3: Vec<_> = v0.iter().map(|x| x + 1).collect();

    println!("{v1:#?}");
    println!("{v2:#?}");
    println!("{v3:#?}");
}
