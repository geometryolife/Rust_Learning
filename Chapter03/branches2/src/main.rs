fn main() {
    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

// The value of number is: 5
