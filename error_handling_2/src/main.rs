use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
// use std::error::Error; to return Result <T, E> type from main

// propagating the error
// fn read_username_from_file() -> Result<String, io::Error>{
//     let username_file_result =File::open("hello.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e)=> return Err(e),
//     };
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username){
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

//using ? operator
fn read_username_from_file() -> Result<String,io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // same as
    /*
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username);
    */
    // or directly
    /*
    fs::read_to_string("hello.txt")
    */

    // `?` operator can also be used to return Option::Some or Option::None.
    //e.g.
    /*
    fn last_char_of_first_line(text:&str)-> Option<char> {
        text.lines().next()?.chars().last()
    }
    */
}
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(file) => file,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
        // unwrap and expect method also exist for Result <t, e> type
    };
}

/*
fn main() -> Result<(), Box<dyn Error>>{
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
*/
