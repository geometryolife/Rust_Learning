// 【例9-7】一个使用 ? 运算符来将错误返回给调用者的函数
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("{:?}", read_username_from_file());
    // === Output ===
    // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}
