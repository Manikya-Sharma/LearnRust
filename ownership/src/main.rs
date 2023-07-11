fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world");
    // println!("{}", s);

    // let x = 5;
    // let y = x;

    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    // println!("{s1} world!");

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
} // x goes out of scope, then s. But since s was moved, nothing special happens

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // String out of scope: drop is called

fn makes_copy(nome_integer: i32) {
    println!("{nome_integer}");
} // integer out of scope, nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
