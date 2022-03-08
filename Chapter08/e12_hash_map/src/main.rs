// 【例8-21】使用队伍列表和分数列表创建哈希映射
use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:#?}", scores);
    // === Output ===
    // {
    //     "Yellow": 50,
    //     "Blue": 10,
    // }
}
