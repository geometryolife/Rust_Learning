fn main() {
    // 复合类型
    // 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // 解构元组

    println!("The value of y is: {}", y);

    // 使用点号结构元组
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_four, one);
}
