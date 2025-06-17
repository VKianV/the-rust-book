fn main() {
    let v = vec![1, 2, 3];

    let does_not_exist = &v[100];
    println!("{does_not_exist}");

    let does_not_exist = v.get(100);
    println!("{does_not_exist:?}");
}
