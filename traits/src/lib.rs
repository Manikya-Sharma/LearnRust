//pub trait Summary {
//    fn summarize(&self) -> String {
//        String::from("Read more")
//    }
//}
//
//pub struct NewsArticle {
//    pub headline: String,
//    pub location: String,
//    pub author: String,
//    pub content: String,
//}
//
//impl Summary for NewsArticle {
//    //    fn summarize(&self)->String {
//    //       format!("{}, by {}({})", self.headline, self.author, self.location)
//    //    }
//}
//
//pub struct Tweet {
//    pub username: String,
//    pub content: String,
//    pub reply: bool,
//    pub retweet: bool,
//}
//
//impl Summary for Tweet {
//    fn summarize(&self) -> String {
//        format!("{}: {}", self.username, self.content)
//    }
//}
//
//pub fn notify(item: &impl Summary) {
//    println!("Breaking News!: {}", item.summarize());
//}

use std::fmt::Display;

pub struct Pair<T>{
   pub x:T,
    pub y:T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y:T)->Self{
        Self{x,y}
    }
}

impl<T: Display + PartialOrd>Pair<T> {
    pub fn cmp_display(&self){
        if self.x >= self.y {
            println!("Larger is x = {}",self.x);
        } else {
            println!("Larger is y = {}", self.y);
        }
    }
}

