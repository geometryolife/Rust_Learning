// 【例10-5】使用泛型参数定义的 largest 函数，目前还无法通过编译
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);
}

// === Output ===
// error[E0369]: binary operation `>` cannot be applied to type `T`
//  --> src/main.rs:6:17
//   |
// 6 |         if item > largest {
//   |            ---- ^ ------- T
//   |            |
//   |            T
//   |
// help: consider restricting type parameter `T`
//   |
// 2 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//   |             ^^^^^^^^^^^^^^^^^^^^^^
