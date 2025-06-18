use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 60);

    // let team = "Blue".to_string();
    // scores.get(&team).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("the team {}'s score is {}", key, value);
    }
}
