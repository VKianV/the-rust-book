fn main() {
    let s1 = String::from("it is kian");
    let len = calculate_length(&s1);
    println!("the length of {s1} is {len}");
}

fn calculate_length(input: &String) -> usize {
    input.len()
}
