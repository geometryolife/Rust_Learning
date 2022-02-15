fn main() {
    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition {
        5
    } else {
        six // if 分支的返回值都必须是相同类型
    };

    println!("The value of number is: {}", number);
}

//   Compiling branches2 v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter03/branches2)
//error[E0425]: cannot find value `six` in this scope
// --> src/main.rs:7:9
//  |
//7 |         six // if 分支的返回值都必须是相同类型
//  |         ^^^ not found in this scope

//error: aborting due to previous error

//For more information about this error, try `rustc --explain E0425`.
//error: could not compile `branches2`

//To learn more, run the command again with --verbose.
