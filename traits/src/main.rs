//use traits::{notify, NewsArticle, Summary, Tweet};
//
//fn main() {
//    let tweet = Tweet {
//        username: String::from("horse_ebooks"),
//        content: String::from("of course as you know"),
//        reply: false,
//        retweet: false,
//    };
//    println!("1 new tweet: {}", tweet.summarize());
//    let article = NewsArticle {
//        headline: String::from("Penguins win championship"),
//        location: String::from("USA"),
//        author: String::from("Iceburgh"),
//        content: String::from("Congrats they won"),
//    };
//    println!("New article available: {}", article.summarize());
//    notify(&tweet);
//}

use traits::Pair;

fn main(){
    let p1 = Pair::new(5, 6); 
    p1.cmp_display();
}

