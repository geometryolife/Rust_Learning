fn main() {
    let a = [1, 2, 3, 4, 5]; // 初始化数组
    let index = 10;

    let element = a[index]; // 非法的数组访问

    println!("The value of element is: {}", element);

    // 注意：旧版本的 rust 能够编译，但是目前我使用的版本无法编译
    // rustc 1.53.0 (53cb7b09b 2021-06-17)
}
