use std::io;

fn main() {
    println!("Which unit temperature to be taken? (c/f)");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    println!("Enter the temperature:-");
    let mut given_temp = String::new();
    io::stdin()
        .read_line(&mut given_temp)
        .expect("Could not read line");
    let given_temp: f32 = match given_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    if choice == "c" { // Condition not working
        let fahrenheit = to_fahrenheit(given_temp);
        println!("The temperature in fahrenheit is {fahrenheit} F");
    } else {
        let celsius = to_celsius(given_temp);
        println!("The temperature in celsius is {celsius} C");
    }
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn to_fahrenheit(celsius: f32) -> f32 {
    (9.0 / 5.0) * celsius + 32.0
}
