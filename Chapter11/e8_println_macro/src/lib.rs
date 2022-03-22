// 【例11-10】测试一个调用了 println! 的函数
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value)
    }
}

// === Output ===
// running 2 tests
// test tests::this_test_will_fail ... FAILED
// test tests::this_test_will_pass ... ok

// failures:

// ---- tests::this_test_will_fail stdout ----
// I got the value 8
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `5`,
//  right: `10`', src/lib.rs:20:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// failures:
//     tests::this_test_will_fail

// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// error: test failed, to rerun pass '--lib'

// 使用 --nocapture 标记来禁用输出捕获功能
// === Output ===
// running 2 tests
// test tests::this_test_will_fail ... I got the value 8
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `5`,
//  right: `10`', src/lib.rs:20:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// FAILED
// test tests::this_test_will_pass ... I got the value 4
// ok

// failures:

// failures:
//     tests::this_test_will_fail

// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// error: test failed, to rerun pass '--lib'

// 我使用的是单核服务器，所以可能效果出不来并行结果，可以尝试在多核心、多线程
// 机器使用 --test-threads=1 和 --nocapture 来运行。
