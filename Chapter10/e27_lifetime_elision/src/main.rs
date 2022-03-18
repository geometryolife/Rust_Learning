// 【例10-26】即使参数和返回类型都是引用，示例 4-9 中定义的这个
// 函数依然没有使用生命周期标注
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("{}", word);
    // === Output ===
    // hello
}

// 在 Rust 早期版本（pre-1.0）中，这样的代码无法通过编译。函数签名必需为：
// fn first_word<'a>(s: &'a str) -> &'a str {}
