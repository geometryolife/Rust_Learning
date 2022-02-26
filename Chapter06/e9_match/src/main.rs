// 【例6-5】一个对 Option<i32> 使用 match 表达式的函数
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}, {:?}", five, six, none);
}

// 匹配必须穷举所有的可能
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
//     // === Output ===
//     // error[E0004]: non-exhaustive patterns: `None` not covered
//     //    --> src/main.rs:18:11
//     //     |
//     // 18  |     match x {
//     //     |           ^ pattern `None` not covered
//     //     |
//     //     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
//     //     = note: the matched value is of type `Option<i32>`
// }
