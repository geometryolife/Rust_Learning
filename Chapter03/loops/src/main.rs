fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 结束并返回值
        }
    }; // 结束当前语句

    println!("The result is {}", result);
}

// The result is 20
