// 【例11-5】使用第 5 章中的 Rectangle 结构体及其 can_hold 方法
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}

// 【例11-6】这个测试会调用 can_hold 来检查一个矩形是否可以容纳另外一个较小的矩形
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // 这个测试断言较小的矩形不能容纳较大的矩形
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

// === Output ===
// ubuntu@VM-12-14-ubuntu:~/other/Rust_Learning/Chapter11/e2_assert/src$ cargo test
//    Compiling e2_assert v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e2_assert)
//     Finished test [unoptimized + debuginfo] target(s) in 0.41s
//      Running unittests (/home/ubuntu/other/Rust_Learning/Chapter11/e2_assert/target/debug/deps/e2_assert-9424b1a220e128c5)

// running 1 test
// test tests::larger_can_hold_smaller ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e2_assert

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// smaller_cannot_hold_larger 测试输出
// === Output ===
// ubuntu@VM-12-14-ubuntu:~/other/Rust_Learning/Chapter11/e2_assert/src$ cargo test
//    Compiling e2_assert v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e2_assert)
//     Finished test [unoptimized + debuginfo] target(s) in 0.86s
//      Running unittests (/home/ubuntu/other/Rust_Learning/Chapter11/e2_assert/target/debug/deps/e2_assert-9424b1a220e128c5)

// running 2 tests
// test tests::larger_can_hold_smaller ... ok
// test tests::smaller_cannot_hold_larger ... ok

// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e2_assert

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 修改 > 为 <，让测试结果出现错误
// === Output ===
// ubuntu@VM-12-14-ubuntu:~/other/Rust_Learning/Chapter11/e2_assert/src$ cargo test
//    Compiling e2_assert v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e2_assert)
//     Finished test [unoptimized + debuginfo] target(s) in 0.84s
//      Running unittests (/home/ubuntu/other/Rust_Learning/Chapter11/e2_assert/target/debug/deps/e2_assert-9424b1a220e128c5)

// running 2 tests
// test tests::larger_can_hold_smaller ... FAILED
// test tests::smaller_cannot_hold_larger ... ok

// failures:

// ---- tests::larger_can_hold_smaller stdout ----
// thread 'main' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:30:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// failures:
//     tests::larger_can_hold_smaller

// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// error: test failed, to rerun pass '--lib'
