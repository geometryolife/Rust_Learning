// 定义 trait
// 【例10-12】Summary trait 由 summarize 方法所提供的行为组成
pub trait Summary {
    fn summarize(&self) -> String; // 方法签名
}

// 为类型实现 trait
// 【例10-13】为 NewsArticle 与 Tweet 类型实现 Summary trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
