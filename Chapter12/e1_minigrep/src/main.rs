// 【例12-1】将命令行参数收集到一个动态数组中并打印出来
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

// === Output ===
// ["/home/ubuntu/other/Rust_Learning/Chapter12/e1_minigrep/target/debug/e1_minigrep"]

// cargo run needle haystack
// === Output ===
// ["/home/ubuntu/other/Rust_Learning/Chapter12/e1_minigrep/target/debug/e1_minigrep", "needle", "haystack"]
