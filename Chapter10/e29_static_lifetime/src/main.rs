// 静态生命周期
fn main() {
    let s: &'static str = "I have a static lifetime.";
    // 等价于
    let s2: &str = "I have a static lifetime.";

    println!("{}", s);
    // === Output ===
    // I have a static lifetime.

    println!("{}", s2);
    // === Output ===
    // I have a static lifetime.
}
