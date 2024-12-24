use std::collections::HashMap;

fn main() {
    let string_value = String::from("hello world hello world hello");
    let mut hash_map = HashMap::new();

    for word in string_value.split_whitespace() {
        let count = hash_map.entry(word).or_insert(0);
        *count += 1;
        println!("{:?} {}", count, word);
    }
}
