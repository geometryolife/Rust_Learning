fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // 表达式，返回 4
    };

    println!("The value of y is: {}", y);
}
