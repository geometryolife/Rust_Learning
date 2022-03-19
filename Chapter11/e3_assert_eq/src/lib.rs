// 【例11-7】使用 assert_eq! 宏对 add_two 函数进行测试jk
pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
// === Output ===
// running 1 test
// test tests::it_adds_two ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e3_assert_eq

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 修改 add_two 为 a + 3，显示断言失败的结果
// === Output ===
// running 1 test
// test tests::it_adds_two ... FAILED

// failures:

// ---- tests::it_adds_two stdout ----
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `4`,
//  right: `5`', src/lib.rs:12:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// failures:
//     tests::it_adds_two

// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
