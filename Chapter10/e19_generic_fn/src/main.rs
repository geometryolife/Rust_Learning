// 【例10-15】largest 函数可以被用于任何实现了 PartialOrd 与 Copy 这两个 trait 的泛型
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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
    // === Output ===
    // The largest number is 100

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);
    // === Output ===
    // The largest char is y
}
