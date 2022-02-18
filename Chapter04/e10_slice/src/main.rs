// 使用切片重构 first_word 函数
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 错误！
    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str { // 字符串切片类型写作 &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // 返回第一个单词的切片
        }
    }

    &s[..] // 如果没有空格，则返回读取的字符串切片
}

// === Output ===
//error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
// --> src/main.rs:7:5
//  |
//5 |     let word = first_word(&s);
//  |                           -- immutable borrow occurs here
//6 |
//7 |     s.clear(); // 错误！
//  |     ^^^^^^^^^ mutable borrow occurs here
//8 |     println!("the first word is: {}", word);
//  |                                       ---- immutable borrow later used here


// 字符串字面量就是切片
// let s = "Hello, world";
