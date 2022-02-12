use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 生成随机数

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // 生成空字符串

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // 调用标准输入句柄的 read_line 方法来获取输入

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!"); // trim 方法去除多余的换行符，parse 方法解析字符串为数值

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // 比较猜测数字与保密数字（随机数）
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
