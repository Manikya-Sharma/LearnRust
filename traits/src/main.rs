pub trait Summary {
    // fn summarize(&self) ->String;
    // can write implementation here for default behavior
    fn summarize(&self) -> String {
        String::from("(Read more..)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// impl Summary for NewsArticle{
//     fn summarize(&self) ->String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

impl Summary for NewsArticle {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you know"),
        reply: false,
        retweet: false,
    };
    println!("I am new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("This is the headline"),
        location: String::from("Location xyz"),
        author: String::from("My self"),
        content: String::from("Some amount of content"),
    };
    println!("New news article available! {}", article.summarize());
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*
*[Important]
Can also write as:-

pub fn notify<T:Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}
So, if we have multiple parameters all implementing Summary trait,
we can use &impl Summary for both, but in case we want both also
to be the same types, we need to use trait bound,
pub fn notify<T:Summary>(item1:&T, item2:&T){...}
*/

/*
* To ensure parameters implement more than one trait, + operator is used
pub fn notify(item: &(impl Summary + Display)){....}
or
pub fn notify<T:Summary+Display>(item: &T){....}

*/

/*
*Trait bound with where clause
fn some_function<T:Display+Clone, U:Clone+Debug>(t:&T, u:&U)->i32{}
to
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Clone + Display,
{...}
*/

fn returns_summarize() ->impl Summary{
    Tweet{
        username:String::from("etc. etc."),
        content:String::from("content"),
        reply:false,
        retweet:false,
    }
    // Must return only one type in all situation, which also implements trait
}

/*
* Trait bounds to conditionally implement methods
*/

use std::fmt::Display;

struct Pair<T>{
    x:T,
    y:T,
}

impl<T> Pair<T>{
    fn new(x:T, y:T) ->Self{
        Self{x, y}
    }
}
impl <T:Display+PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest member is {}", self.x);
        } else {
            println!("The largest member is {}", self.y);
        }
    }
}

/*
*Blanket implementation
To implement a particular trait on type implementing another trait
e.g.

impl<T:Display> ToString for T{
    ...
}
*/


/*
*How a crate would use out aggregator library

use aggregator::{Summary, Tweet}
fn main(){
    let tweet = Tweet{
        username:String::from("horse_ebooks"),
        content:String::from("of course, as you know"),
        reply:false,
        retweet:false,
    };
    println!("I am new tweet: {}", tweet.summarize());

}



*/
