use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 生成随机数

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // 生成空字符串

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // 调用标准输入句柄的 read_line 方法来获取输入

        let guess: u32 = match guess.trim().parse() { // trim 方法去除多余的换行符，parse 方法解析字符串为数值
            Ok(num) => num, // match 表达式匹配合适的分支并执行相应的代码
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // 比较猜测数字与保密数字（随机数）
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 猜对后退出循环
            }
        }
    }
}
