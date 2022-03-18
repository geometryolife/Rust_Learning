// 【例10-20】main 函数会调用 longest 函数来找到两个字符串切片中较长的一个
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

// 【10-21】用于返回两个字符串切片中较长的那一个的 longest 函数，但
// 目前还无法通过编译
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// === Output ===
// error[E0106]: missing lifetime specifier
//   --> src/main.rs:13:33
//    |
// 13 | fn longest(x: &str, y: &str) -> &str {
//    |               ----     ----     ^ expected named lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
//    |
// 13 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//    |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^

// &i32 // 引用
// &'a i32 // 拥有显式生命周期的引用
// &'a mut i32 // 拥有显式生命周期的可变引用

// 【例10-22】longest 函数的定义指定了签名中所有的引用都必须拥有相同的生命周期 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// === Output ===
// The longest string is abcd
