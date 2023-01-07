use std::io;
fn main() {
    println!("Enter the term:");
    let mut term = String::new();
    io::stdin()
        .read_line(&mut term)
        .expect("Could not read line");
    let term: u32 = match term.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    let final_ans = fibonacci(term);
    println!("The {term} term is {final_ans}");
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
