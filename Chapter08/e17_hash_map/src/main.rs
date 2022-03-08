// 【例8-26】使用哈希映射来存储并计算单词出现的次数
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // === Output ===
    // {"world": 2, "hello": 1, "wonderful": 1}
}
