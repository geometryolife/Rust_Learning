// 【例8-9】遍历动态数组中所有元素的可变引用
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    println!("{:?}", v);

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}

// === Output ===
// [1, 2, 3, 4, 5]
// [51, 52, 53, 54, 55]
