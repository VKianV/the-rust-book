use std::collections::HashMap;
fn main() {
    let text = "By default, HashMap uses a hashing function called SipHash that can provide resistance to denial-of-service (DoS) attacks involving hash tables1.";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:#?}")
}
