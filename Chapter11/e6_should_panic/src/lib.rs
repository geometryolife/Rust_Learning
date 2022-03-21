pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            // panic!(
            //     "Guess value must be greater than or equal to 1, got {}.",
            //     value
            // );
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            // panic!(
            //     "Guess value must be less than or equal to 100, got {}.",
            //     value
            // );
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
// 测试通过的输出
// === Output ===
// running 1 test
// test tests::greater_than_100 ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e6_should_panic

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 向代码引入 Bug，测试失败
// === Output ===
// running 1 test
// test tests::greater_than_100 ... FAILED

// failures:

// ---- tests::greater_than_100 stdout ----
// thread 'main' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:21:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// note: panic did not contain expected string
//       panic message: `"Guess value must be greater than or equal to 1, got 200."`,
//  expected substring: `"Guess value must be less than or equal to 100"`

// failures:
//     tests::greater_than_100

// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// error: test failed, to rerun pass '--lib'
