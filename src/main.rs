fn main() {
    let v = vec![1,2,3,4,5];

    let third = &v[2];
    println!("the third element is {}",third);

    let third = v.get(2);

    match third{
        Some(third) => println!("the element is {}",third),
        None => println!("there was no third element")
    }
}
