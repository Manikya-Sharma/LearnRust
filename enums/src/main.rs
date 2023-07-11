// enum IpAddrKind {
//     V4,
//     V6,
// }
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(IpAddrKind::V4);
//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };
//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
//     let m = Message::Write(String::from("Hello"));
//     m.call();
// }
// fn route(_: IpAddrKind) {}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {
//         println!("{:?}", self);
//     }
// }

// fn main() {
//     let coin: Coin = Coin::Penny;
//     println!("Value = {}", value_in_cents(coin));
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State: {:?}", state);
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(x) => Some(x + 1),
//         None => None,
//     }
// }

fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The max is {max}"),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("Max is {max}");
    }
}
