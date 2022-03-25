// 【例11-11】3 个不同名称的测试
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
}
// === Output ===
// running 3 tests
// test tests::add_three_and_two ... ok
// test tests::add_two_and_two ... ok
// test tests::one_hundred ... ok

// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e9_subset_test

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 使用 cargo test one_hundred 来运行单个测试
// === Output ===
// running 1 test
// test tests::one_hundred ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

// 通过过滤名称来运行多个测试 cargo test add
// === Output ===
// running 2 tests
// test tests::add_three_and_two ... ok
// test tests::add_two_and_two ... ok

// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

// 通过 ignore 属性显式指定来忽略某些测试
// === Output ===
// running 4 tests
// test tests::add_three_and_two ... ok
// test tests::add_two_and_two ... ok
// test tests::expensive_test ... ignored
// test tests::one_hundred ... ok

// test result: ok. 3 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e9_subset_test

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 使用 cargo test -- --ignored 来单独运行这些被忽略的测试
// === Output ===
// running 1 test
// test tests::expensive_test ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

//    Doc-tests e9_subset_test

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
