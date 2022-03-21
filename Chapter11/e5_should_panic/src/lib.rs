// 【例11-8】测试一个应当引发 panic! 的条件
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // if value < 1 || value > 100 {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, get {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
// === Output ===
// running 1 test
// test test::greater_than_100 ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e5_should_panic

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 不发生 panic 则无法通过测试
// === Output ===
// running 1 test
// test test::greater_than_100 ... FAILED

// failures:

// ---- test::greater_than_100 stdout ----
// note: test did not panic as expected

// failures:
//     test::greater_than_100

// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// error: test failed, to rerun pass '--lib'
