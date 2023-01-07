/*
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn die_roll(diceRoll: u32) -> u32 {
    match diceRoll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => reroll(),
        other => move_player(other),
    }
}
fn add_fancy_hat() -> u32 {
    100
}
fn remove_fancy_hat() -> u32 {
    0
}
fn move_player(num_spaces: u32) -> u32 {
    num_spaces
}

fn main() {}
*/

fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }

    if let Some(max) = config_max{
        println!("The maximum is configured to be {max}");
    }
}
