// 【例8-24】替换使用特定键存储的值
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
    // === Output ===
    // {"Blue": 25}
}
