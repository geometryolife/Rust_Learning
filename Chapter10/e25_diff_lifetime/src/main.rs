// 【例10-24】尝试在 string2 离开作用域后使用 result
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// === Output ===
// error[E0597]: `string2` does not live long enough
//  --> src/main.rs:7:44
//   |
// 7 |         result = longest(string1.as_str(), string2.as_str());
//   |                                            ^^^^^^^ borrowed value does not live long enough
// 8 |     }
//   |     - `string2` dropped here while still borrowed
// 9 |     println!("The longest string is {}", result);
//   |                                          ------ borrow later used here

// 返回第一个而不是最长的那个字符串切片参数，那么无需为 y 指定生命周期
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }
// === Output ===
// The longest string is long string is long

// 返回值的生命周期没有与任何参数的生命周期产生关联
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
// === Output ===
// error[E0515]: cannot return value referencing local variable `result`
//   --> src/main.rs:39:5
//    |
// 39 |     result.as_str()
//    |     ------^^^^^^^^^
//    |     |
//    |     returns a value referencing data owned by the current function
//    |     `result` is borrowed here
