// 【例6-6】这里的 match 只在值为 Some(3) 时执行特定的代码
fn main() {
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // 为满足穷尽性，必须添加的表达
    }

    // 使用 if let 实现，if let 可以看作是 match 的语法糖
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
