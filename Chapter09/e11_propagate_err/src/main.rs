// 【例9-9】使用 fs::read_to_string 读取文件
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    println!("{:?}", read_username_from_file());
    // === Output ===
    // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}
