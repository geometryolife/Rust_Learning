fn main() {
    another_function(5);
}

fn another_function(x: i32) { // 函数签名必须显式地声明每个参数的类型
    println!("The value of x is: {}", x);
}
