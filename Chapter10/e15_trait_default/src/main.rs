use e15_trait_default::Summary;
use e15_trait_default::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // === Output ===
    // 1 new tweet: (Read more from @horse_ebooks...)
}
