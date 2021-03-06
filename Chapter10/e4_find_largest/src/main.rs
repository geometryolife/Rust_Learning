// 【例10-4】两个只在名称和签名类型上有所区别的函数
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);

    println!("The largest number is {}", result);
    // === Output ===
    // The largest number is 100

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);

    println!("The largest char is {}", result);
    // === Output ===
    // The largest char is y
}
