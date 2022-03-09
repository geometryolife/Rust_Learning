// 【例9-1】尝试越界访问动态数组中的元素，这会导致 panic!
fn main() {
    let v = vec![1, 2, 3];

    v[99];
    // === Output ===
    //    Compiling e2_panic v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter09/e2_panic)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.62s
    //      Running `/home/ubuntu/other/Rust_Learning/Chapter09/e2_panic/target/debug/e2_panic`
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:5:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // 使用环境变量 RUST_BACKTRACE 来获取回溯信息
    // 【例9-2】当环境变量 RUST_BACKTRACE 被设置好后，通过调用 panic! 所生成的回溯
    // === Output ===
    // ubuntu@geometryolife:~/other/Rust_Learning/Chapter09/e2_panic/src$ RUST_BACKTRACE=1 cargo run
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    //      Running `/home/ubuntu/other/Rust_Learning/Chapter09/e2_panic/target/debug/e2_panic`
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:5:5
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/std/src/panicking.rs:493:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/panicking.rs:92:14
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/panicking.rs:69:5
    //    3: <usize as core::slice::index::SliceIndex<[T]>>::index
    //              at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/slice/index.rs:184:10
    //    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
    //              at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/slice/index.rs:15:9
    //    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    //              at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/alloc/src/vec/mod.rs:2384:9
    //    6: e2_panic::main
    //              at ./main.rs:5:5
    //    7: core::ops::function::FnOnce::call_once
    //              at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/ops/function.rs:227:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
}
