// 将字符串切片作为参数
fn main() {
    let my_string = String::from("hello world");

    // first_word1 可以接收 String 对象的切片作为参数
    let word = first_word2(&my_string[..]);
    println!("String way: {}", word);
    // === Output ===
    // String way: hello

    let my_string_literal = "hello world";

    // first_word2 可以接收字符串字面量的切片作为参数
    let word = first_word2(&my_string_literal[..]);
    println!("literal slice way: {}", word);
    // === Output ===
    // literal slice way: hello

    // 由于字符串字面量本身就是切片，所以我们可以在这里直接将它传入函数，
    // 而不需要使用额外的切片语法！
    let word = first_word2(my_string_literal);
    println!("literal way: {}", word);
    // === Output ===
    // literal way: hello
}

// 目前的函数签名
// 只能使用 String 类型的引用
fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// 改进后的函数签名
// 可以同时处理 String 和 &str
// 更加通用，且不会损失任何功能
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}


// 其他类型的切片，以数组为例
// 切片类型：&[i32] -> (&element, len)
// let a = [1, 2, 3, 4, 5];

// let slice = &a([1..3]);
