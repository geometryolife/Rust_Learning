// 【例8-6】尝试在只有 5 个元素的动态数组中访问索引值为 100 的元素
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    // === Output ===
    // warning: unused variable: `does_not_exist`
    //  --> src/main.rs:4:9
    //   |
    // 4 |     let does_not_exist = &v[100];
    //   |         ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_does_not_exist`
    //   |
    //   = note: `#[warn(unused_variables)]` on by default

    // warning: 1 warning emitted

    //     Finished dev [unoptimized + debuginfo] target(s) in 0.28s
    //      Running `/home/ubuntu/other/Rust_Learning/Chapter08/e3_vector/target/debug/e3_vector`
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', src/main.rs:4:27
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let does_not_exist = v.get(100);
    // === Output ===
    // warning: unused variable: `does_not_exist`
    //   --> src/main.rs:21:9
    //    |
    // 21 |     let does_not_exist = v.get(100);
    //    |         ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_does_not_exist`
    //    |
    //    = note: `#[warn(unused_variables)]` on by default

    // warning: 1 warning emitted

    //     Finished dev [unoptimized + debuginfo] target(s) in 0.27s
    //      Running `/home/ubuntu/other/Rust_Learning/Chapter08/e3_vector/target/debug/e3_vector`
}
