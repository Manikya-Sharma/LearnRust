use std::io;
fn main() {
    let x = 5_000;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is {x}");
    }
    println!("The value of x is {x}");

    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // let five_hundred = tup.0;
    // let array = [1, 2, 3, 4, 5];
    // let b_array = [3; 5]; // [3,3,3,3,3]
    let a = [1, 2, 3, 4, 5];
    println!("Please enter array index:");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of element at index {index} is {element}");
}
