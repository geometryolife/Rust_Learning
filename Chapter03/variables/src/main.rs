fn main() {
    let x = 5; // 创建不可变变量，并将其绑定到 5 上
    println!("The value of x is: {}", x);
    x = 6; // 尝试给不可变变量二次赋值
    println!("The value of x is: {}", x);
}
