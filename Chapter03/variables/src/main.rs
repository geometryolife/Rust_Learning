fn main() {
    let guess = "42".parse().expect("Not a number!"); // 移除类型标注，编译器无法自动推导出变量的类型
}
