// 假设有一个函数会接收客人姓名作为参数，并返回拼接的问候语作为结果
pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Joe");
    //     assert!(result.contains("Joe"));
    // }

    // 添加自定定义错误提示信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Joe");

        assert!(
            result.contains("Joe"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
// 包含指定名字的测试
// === Output ===
// running 1 test
// test tests::greeting_contains_name ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e4_custom

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 不包含名字时的测试
// === Output ===
// running 1 test
// test tests::greeting_contains_name ... FAILED

// failures:

// ---- tests::greeting_contains_name stdout ----
// thread 'main' panicked at 'assertion failed: result.contains(\"Joe\")', src/lib.rs:14:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// failures:
//     tests::greeting_contains_name

// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 自定义错误信息后，看到更具实际意义的错误提示信息
// === Output ===
// running 1 test
// test tests::greeting_contains_name ... FAILED

// failures:

// ---- tests::greeting_contains_name stdout ----
// thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:22:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// failures:
//     tests::greeting_contains_name

// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
