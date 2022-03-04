// 【例8-5】使用索引或 get 方法来访问动态数组中的元素
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);
    // === Output ===
    // The third element is 3

    // 接收索引为参数的 get 方法会返回一个 Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // === Output ===
    // The third element is 3
}
