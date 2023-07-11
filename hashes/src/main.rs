use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");
    println!("-------------------");
    for (key, value) in &scores {
        println!("{key}:{value}");
    }
    println!("-------------------");
    let field_name = String::from("Favourite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Overwriting a value
    scores.insert(String::from("Blue"), 20);
    for (k, v) in &scores {
        println!("{k}:{v}");
    }
    println!("-------------------");
    // Add only if present
    scores.entry(String::from("Yellow")).or_insert(80);
    scores.entry(String::from("Green")).or_insert(80);
    for (k, v) in &scores {
        println!("{k}:{v}");
    }
    println!("-------------------");
    // update old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (k, v) in &map {
        println!("{k}:{v}");
    }
    println!("-------------------");
}
