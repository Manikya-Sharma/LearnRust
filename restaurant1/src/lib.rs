mod front_of_house;

// use crate::front_of_house::hosting; // Can't use in more than one scopes
// pub use crate:: .... is re-export allowing other modules to also use it.

// pub fn eat_at_restaurant() {
////     crate::front_of_house::hosting::add_to_waitlist();
//  hosting::add_to_waitlist();
//     // front_of_house::hosting::add_to_waitlist(); // Relative path
//     // super:: is same as ../
// }

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruits: String,
    }
    //enum pub is complete pub

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I would like {} toast please!", meal.toast)

    //meal.seasonal_fruit = something wont work
}
