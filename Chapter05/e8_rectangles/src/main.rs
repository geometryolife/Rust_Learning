// 使用元组来重构代码
// 【例5-9】通过元组来指定长方形的宽度和高度
fn main() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// === Output ===
// The area of the rectangle is 1500 square pixels.
