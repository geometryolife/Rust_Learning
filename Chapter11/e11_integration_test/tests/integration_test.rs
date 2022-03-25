use e11_integration_test as adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// === Output ===
//    Compiling e11_integration_test v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e11_integration_test)
//     Finished test [unoptimized + debuginfo] target(s) in 0.68s
//      Running unittests (/home/ubuntu/other/Rust_Learning/Chapter11/e11_integration_test/target/debug/deps/e11_integration_test-bc81b678
// 7bc01ce0)

// running 1 test
// test tests::internal ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running tests/integration_test.rs (/home/ubuntu/other/Rust_Learning/Chapter11/e11_integration_test/target/debug/deps/integration_t
// est-f08b7764dd5a982a)

// running 1 test
// test it_adds_two ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests e11_integration_test

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// -- test 指定文件名，可以单独运行某个特定集成测试文件下的所有测试函数
// === Output ===
// ubuntu@VM-12-14-ubuntu:~/other/Rust_Learning/Chapter11/e11_integration_test/tests$ cargo test --test integration_test
//    Compiling e11_integration_test v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter11/e11_integration_test)
//     Finished test [unoptimized + debuginfo] target(s) in 0.47s
//      Running tests/integration_test.rs (/home/ubuntu/other/Rust_Learning/Chapter11/e11_integration_test/target/debug/deps/integration_t
// est-f08b7764dd5a982a)

// running 1 test
// test it_adds_two ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
