fn main() {
    let number = 7;

    if number { // Rust 不会自动尝试将非布尔类型转换为布尔类型
        println!("condition was true"); // 分支（arm）
    } else {
        println!("condition was false");
    }
}

//   Compiling branches v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter03/branches)
//error[E0308]: mismatched types
// --> src/main.rs:4:8
//  |
//4 |     if number { // Rust 不会自动尝试将非布尔类型转换为布尔类型
//  |        ^^^^^^ expected `bool`, found integer

//error: aborting due to previous error

//For more information about this error, try `rustc --explain E0308`.
//error: could not compile `branches`

//To learn more, run the command again with --verbose.
