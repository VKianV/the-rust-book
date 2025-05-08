fn main() {
    let s = String::from("it is kian");
    let (s2, len) = calculate_length(s);
    println!("the length of {s2} is {len}");
}

fn calculate_length(input: String) -> (String, usize) {
    let length = input.len();
    (input, length)
}
