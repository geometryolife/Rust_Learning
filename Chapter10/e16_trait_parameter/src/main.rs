use e16_trait_parameter::Summary;
use e16_trait_parameter::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // === Output ===
    // 1 new tweet: horse_ebooks: of course, as you probably already know, people

    notify(tweet);
    // === Output ===
    // Breaking news! horse_ebooks: of course, as you probably already know, people

    notify_full(tweet2);
    // === Output ===
    // Breaking news! horse_ebooks: of course, as you probably already know, people
}

// 使用 trait 作为参数
// trait 约束的一种语法糖
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait 约束的完整形式
pub fn notify_full<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 使用场景指导
// 同时实现了 Summary trait 的不同类型
// pub fn notify(item1: impl Summary, item2: impl Summary) {}

// 强迫两个参数使用同样的类型
// pub fn notify<T: Summary>(item1: T, item2: T) {}

// 通过 + 语法来指定多个 trait 约束
// pub fn notify(item: impl Summary + Display) {}
// pub fn notify<T: Summary + Display>(item: T) {}

// 使用 where 从句来简化 trait 约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

// fn some_function<T, U>(t: T, u: U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }
