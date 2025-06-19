use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);

    scores.entry("Blue".to_string()).or_insert(50);
    scores.entry("Yellow".to_string()).or_insert(50);

    println!("{scores:?}")
}
