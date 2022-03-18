// 【例10-23】使用具有不同生命周期的 String 来调用 longest 函数
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// === Output ===
// The longest string is long string is long
