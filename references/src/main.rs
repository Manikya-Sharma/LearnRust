fn main() {
    // let s1 = String::from("Hello");
    // let len = calculate_length(&s1);
    // println!("The length is {s1} is {len}.");
    // // changing borrowed part causes error
    // let mut s = String::from("Hello");
    // let s2 = &mut s;
    // let s3 = &mut s;
    // println!("{s2}, {s3}"); // not allowed (basically cannot simultaneosly have two mutable references)
    // change(&mut s);

    // Dangling pointer
    // let reference_to_nothing = dangle();
    let my_string = String::from("Hello me");
    let i = first_word(&my_string);
    println!("{i}");
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(s: &mut String) {
//     s.push_str(", world");
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
