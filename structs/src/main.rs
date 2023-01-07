fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("Change the username");

    /*
    let user2 = User {
        username: user1.username,
        email: String::from("anotherexample@123.com"),
        sign_in_count: user1.sign_in_count,
    };
    */
    let user3 = User {
        email: String::from("Anotherone@123"),
        ..user1
    };

    let black = Color(0,0,0); // tuple struct
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

