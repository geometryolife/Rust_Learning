// 定义方法
// 【例5-13】在 Rectangle 结构体中定义 area 方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    // === Output ===
    // The area of the rectangle is 1500 square pixels.
}
