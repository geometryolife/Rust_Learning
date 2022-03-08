// 【例8-25】通过使用 entry 方法在键不存在对应值时插入数据
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // === Output ===
    // {"Blue": 10, "Yellow": 50}
}
