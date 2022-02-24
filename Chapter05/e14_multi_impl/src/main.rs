// 多个 impl 块：每个结构体可以拥有多个 impl 块
// 【例5-16】使用多个 impl 块来重写【例5-15】
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The area of the rectangle is {} square pixels.", rect2.area());
    println!("The area of the rectangle is {} square pixels.", rect3.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// 计算面积的 impl 块
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 判断包含关系的 impl 块
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// === Output ===
// The area of the rectangle is 1500 square pixels.
// The area of the rectangle is 400 square pixels.
// The area of the rectangle is 2700 square pixels.
// Can rect1 hold rect2? true
// Can rect1 hold rect3? false
