// Q：编写一个搜索函数，它接收字符串作为参数，并将字符串中的首个单词作为
// 结果返回。如果字符串中不存在空格，那么就意味着整个字符串是一个单词，
// 直接返回整个字符串作为结果即可。

// 切片
// 一种不持有所有权的数据类型
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 索引 5 会被绑定到变量 word 上

    s.clear(); // 这里的 clear 方法会清空当前字符串，使之变为 ""

    // 虽然 word 依然拥有 5 这个值，但因为我们用于搜索的字符串发生了改变，
    // 所以这个索引也就没有任何意义了，word 到这里便失去了有效性

    println!("{}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // as_bytes 方法将 String 转换为字节数组

    // iter 方法创建一个可以遍历字节数组的迭代器
    // enumerate 方法将 iter 的每个输出作为元素逐一封装在对应的元组中返回 -> (index, &element)
    // for 循环中的解构模式：(i, &item) => 使用模式匹配解构元组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // 搜索到空格时，返回当前位置的索引
        }
    }

    s.len() // 搜索失败时返回传入字符串的长度
}

// === Output ===
// 5
