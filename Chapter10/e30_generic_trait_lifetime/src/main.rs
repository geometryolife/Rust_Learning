// 同时使用泛型参数、trait 约束与生命周期
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(string1.as_str(), string2, "Which is longest?");
    println!("The longest string is {}", result);
    // === Output ===
    // Announcement! Which is longest?
    // The longest string is abcd
}
