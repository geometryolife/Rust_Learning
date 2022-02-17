// 利用元组来同时返回多个值
// 示例4-5：返回参数的所有权
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // 将函数的元组返回值解构

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len 方法返回当前字符串的长度

    (s, length) // 返回元组
}
// === Output ===
// The length of 'hello' is 5.
