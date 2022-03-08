// 【例8-20】创建一个新的哈希映射并插入一些键值对
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);
    // === Output ===
    // {
    //     "Blue": 10,
    //     "Yellow": 50,
    // }
}
