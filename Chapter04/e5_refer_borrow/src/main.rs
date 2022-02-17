// 引用：在不获取所有权的前提下使用值
// 使用引用重构示例4-5
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}', is {}", s1, len);
}

fn calculate_length(s: &String) -> usize { // s 是一个指向 String 的引用
    s.len()
} // 到这里，s 离开作用域。但是由于它并不持有自己所指向值的所有权，
// 所以没有什么特殊的事情会发生

// === Output ===
// The length of 'hello', is 5
