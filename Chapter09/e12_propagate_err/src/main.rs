use std::fs::File;

fn main() {
    // ? 运算符只能被用于返回 Result 的函数
    let f = File::open("hello.txt")?;
    // === Output ===
    // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `Try`)
    //  --> src/main.rs:5:13
    //   |
    // 3 | / fn main() {
    // 4 | |     // ? 运算符只能被用于返回 Result 的函数
    // 5 | |     let f = File::open("hello.txt")?;
    //   | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
    // 6 | |     // === Output ===
    // 7 | | }
    //   | |_- this function should return `Result` or `Option` to accept `?`
    //   |
    //   = help: the trait `Try` is not implemented for `()`
    //   = note: required by `from_error`
}
