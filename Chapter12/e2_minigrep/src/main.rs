// 【例12-2】创建变量来存储查询参数和文件名参数
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

// === Output ===
// Searching for test
// In file sample.txt
