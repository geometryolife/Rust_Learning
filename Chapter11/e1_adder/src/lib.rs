// 【例11-1】运行 cargo new 命令自动生成的测试模块和测试函数
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
// 【例11-2】运行生成的模版测试后所输出的结果
// === Output ===
// ubuntu@VM-12-14-ubuntu:~/other/Rust_Learning/Chapter11/e1_adder/src$ cargo test
//    Compiling e1_adder v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e1_adder)
//     Finished test [unoptimized + debuginfo] target(s) in 1.42s
//      Running unittests (/home/ubuntu/other/Rust_Learning/Chapter11/e1_adder/target/debug/deps/e1_adder-b56fa871d5ce7f46)

// running 1 test
// test tests::it_works ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e1_adder

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 修改测试函数名不会修改 unittest 可执行文件名
// ubuntu@VM-12-14-ubuntu:~/other/Rust_Learning/Chapter11/e1_adder/src$ cargo test
//    Compiling e1_adder v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e1_adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.38s
//      Running unittests (/home/ubuntu/other/Rust_Learning/Chapter11/e1_adder/target/debug/deps/e1_adder-b56fa871d5ce7f46)

// running 1 test
// test tests::exploration ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e1_adder

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 【例11-3】增加一个新的测试，她会因为调用 panic! 宏而运行失败
#[test]
fn another() {
    panic!("Make this test fail");
}

// 【例11-4】测试结果显示一个测试通过、一个测试失败
// Small Easter eggs: Chinese => Opt + Shift + k => []
// === Output ===
// ubuntu@VM-12-14-ubuntu:~/other/Rust_Learning/Chapter11/e1_adder/src$ cargo test
//    Compiling e1_adder v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e1_adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.41s
//      Running unittests (/home/ubuntu/other/Rust_Learning/Chapter11/e1_adder/target/debug/deps/e1_adder-b56fa871d5ce7f46)

// running 2 tests
// test another ... FAILED
// test tests::exploration ... ok

// failures:

// ---- another stdout ----
// thread 'main' panicked at 'Make this test fail', src/lib.rs:47:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// failures:
//     another

// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// error: test failed, to rerun pass '--lib'
