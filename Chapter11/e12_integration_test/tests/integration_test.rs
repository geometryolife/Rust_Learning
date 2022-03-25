use e12_integration_test as adder;

mod common; // 引用 common 模块

#[test]
fn it_adds_two() {
    // 调用模块函数
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
// 此时没有 common 模块的干扰了
// === Output ===
// running 1 test
// test tests::internal ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running tests/integration_test.rs (/home/ubuntu/other/Rust_Learning/Chapter11/e12_integration_test/target/debug/deps/integration_t
// est-55847ebe383f2618)

// running 1 test
// test it_adds_two ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e12_integration_test

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
