// 失败时触发 panic 的快捷方式：unwrap 和 expect
use std::fs::File;

fn main() {
    // let f = File::open("hello.txt").unwrap();
    // === Output ===
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.62s
    //      Running `/home/ubuntu/other/Rust_Learning/Chapter09/e7_unwrap_expect/target/debug/e7_unwrap_expect`
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directo
    // ry" }', src/main.rs:5:37
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // === Output ===
    // thread 'main' panicked at 'Failed to open hello.txt: Os { code: 2, kind: NotFound,
    // message: "No such file or directory" }', src/main.rs:13:37
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
