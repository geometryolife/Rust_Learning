// 【例9-6】使用 match 将错误返回给调用者的函数
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("{:?}", read_username_from_file());
    // === Output ===
    // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}
