// 【例9-4】使用 match 表达式来处理所有可能的 Result 变体
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

// === Output ===
//     Finished dev [unoptimized + debuginfo] target(s) in 0.28s
//      Running `/home/ubuntu/other/Rust_Learning/Chapter09/e4_open_file/target/debug/e4_open_file`
// thread 'main' panicked at 'There was a problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }',
//  src/main.rs:10:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
