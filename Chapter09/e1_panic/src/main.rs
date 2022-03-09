fn main() {
    panic!("crash and burn");
    // panic 默认行为为栈展开
    // === Output ===
    //    Compiling e1_panic v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter09/e1_panic)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.28s
    //      Running `/home/ubuntu/other/Rust_Learning/Chapter09/e1_panic/target/debug/e1_panic`
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // 在 Cargo.toml 配置在遇到 panic 时终止程序
    // === Output ===
    // error: failed to parse manifest at `/home/ubuntu/other/Rust_Learning/Chapter09/e1_panic/Cargo.toml`

    // Caused by:
    //   invalid type: string "abort", expected struct TomlProfile for key `profile.panic`
}
