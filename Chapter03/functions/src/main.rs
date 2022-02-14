fn main() {
    another_function(5, 6);
}

// 声明多个参数
fn another_function(x: i32, y: i32) { // 函数签名必须显式地声明每个参数的类型
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
