fn main() {
    another_function(5);
    print_labelled_measurement(5, 'h');
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
    let x = five();
    println!("The value of x is {x}");
    let p1 = plus_one(5);
    println!("The value of p1 is {p1}");
}

fn another_function(x: i32) {
    println!("THe value of x is {x}");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measured value is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
