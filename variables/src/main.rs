use std::io;

fn main() {
    /* let mut x = 5;
     println!("The value of x is: {x}");

     x = 6;
     println!("The value of x is: {x}");

    */
    /*
    let x = 5;

    let x = x+1;
    {
        let x = x*2;
        println!("The value of x in inner scope is {x}");
    }
    println!("The value of x in outer scope is {x}");
    */

    /*
    let spaces = "     ";
    let spaces = spaces.len();

    println!("{spaces}");
     */

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x is {x}, y is {y}, z is {z}");

    /*
       let a = [1,2,3,4,5]; //array
       let b = [3;5];
    */

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index:");

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
