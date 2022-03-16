use e17_trait_return::Summary;
use e17_trait_return::Tweet;

fn main() {
    returns_summarizable();
}

// returns_summarizable 返回了一个 Tweet，但调用者却无法知晓这一信息
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
