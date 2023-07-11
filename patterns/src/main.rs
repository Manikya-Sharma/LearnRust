// fn main() {
//     let favorite_color: Option<&str> = None;
//     let is_tuesday = false;
//     let age: Result<u8, _> = "34".parse();
//
//     if let Some(color) = favorite_color {
//         println!("Using your favorite color, {color}, as the background");
//     } else if is_tuesday {
//         println!("Tuesday is green day!");
//     } else if let Ok(age) = age {
//         if age > 30 {
//             println!("Using purple as the background color");
//         } else {
//             println!("Using orange as the background color");
//         }
//     } else {
//         println!("Using blue as the background color");
//     }
//
//     let mut stack = Vec::new();
//
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
//
//     let v = vec!['a', 'b', 'c'];
//     for (index, value) in v.iter().enumerate() {
//         println!("{} is at index {}", value, index);
//     }
//
//     let point = (3, 5);
//     print_coordinates(&point);
// }
//
// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current location: ({}, {})", x, y);
// }
//

fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let p = Point { x: 0, y: 4 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(4, b);
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    enum NewMessage {
        Hello { id: i32 },
    }
    let msg = NewMessage::Hello { id: 5 };

    match msg {
        NewMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found id in range: {}", id_variable),
        NewMessage::Hello { id: 10..=12 } => {
            println!("Found id in another range")
        }
        NewMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
