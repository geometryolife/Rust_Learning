// 【例9-3】打开一个文件
use std::fs::File;

fn main() {
    let f = File::open("hello.txt"); // 返回类型 Result<T, E>

    // 向编译器索要答案！
    // let f: u32 = File::open("hello.txt");
    // === Output ===
    // error[E0308]: mismatched types
    //  --> src/main.rs:8:18
    //   |
    // 8 |     let f: u32 = File::open("hello.txt");
    //   |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `Result`
    //   |            |
    //   |            expected due to this
    //   |
    //   = note: expected type `u32`
    //              found enum `Result<File, std::io::Error>`
}
