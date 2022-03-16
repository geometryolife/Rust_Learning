use e13_trait::Summary;
use e13_trait::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // === Output ===
    // 1 new tweet: horse_ebooks: of course, as you probably already know, people
}
