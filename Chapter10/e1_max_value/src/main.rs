// 【例10-1】在一个数字列表中找到最大值
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    // === Output ===
    // The largest number is 100
}
