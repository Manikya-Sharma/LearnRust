// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername"),
//         email: String::from("someemail@example.com"),
//         sign_in_count: 1,
//     };
//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     let user3 = User {
//         email: String::from("A string"),
//         ..user1
//     };
//     // println!("{}", user1.email);
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn buildUser(email: String, username: String) -> User {
//     User {
//         active: true,
//         email,
//         username,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of rectangle is {} square pixels",
//         area(width1, height1)
//     );
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect1 = (30, 50);
//     println!("The area of rectanlge is {} square units", area(rect1));
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    let sq = Rectangle::square(50);
    println!("Area of square`: {}", sq.area());
    // println!("Rectangle:{:#?}", rect1);
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
