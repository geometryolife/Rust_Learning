// 【例9-8】? 运算符后面的链式方法调用
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    println!("{:?}", read_username_from_file());
    // === Output ===
    // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}
