fn main() {
    println!("Hello, world!");
    another_function(5);

    print_labeled_measurements(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    let f = five();
    println!("Five is {f}");

    let pf = plus_one(f);
    println!("Plus one: {pf}");
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: u32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
