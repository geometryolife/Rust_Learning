use std::collections::HashMap;

fn main() {
    // 【例8-23】访问存储在哈希映射中的蓝队分数
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);
    // === Output ===
    // Some(10)

    // 遍历 HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // === Output ===
    // Yellow: 50
    // Blue: 10
}
