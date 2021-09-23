use std::collections::HashMap;

fn main() {
    // hashmap is good when not using an index use case
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:#?}", score);

    // iterating over each key / value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn vector_tuples_to_hashmap() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // we can use <_, _> because Rust will infer the type as <String, i32>
    let mut scores: HashMap<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter()).collect();
}

fn ownership_and_hashmaps() {
    let field_name = String::from("favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
}
