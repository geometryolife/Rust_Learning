// 使用结构体来重构代码：增加有意义的描述信息
// 【例5-10】定义 Rectangle 结构体
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

// area 函数准确无误地描述了计算正方形面积的意图
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// === Output ===
// The area of the rectangle is 1500 square pixels.
