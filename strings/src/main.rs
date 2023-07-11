fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = String::from(data);
    println!("{s}, {s2}, {s3}");
    let hello = String::from("こんにちは");
    println!("{}", hello);
    s.push_str("Hello World");
}