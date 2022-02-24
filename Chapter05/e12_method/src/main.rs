// 任务：检测当前的 Rectangle 实例是否能完整包含传入的另一
// 个 Rectangle 实例，如果是的话就返回 true，否则返回 false。
struct Rectangle {
    width: u32,
    height: u32,
}

// 【例5-14】使用 can_hold 方法
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // === Output ===
    // Can rect1 hold rect2? true
    // Can rect1 hold rect3? false
}

// 【例5-15】基于 Rectangle 实现 can_hold 方法，
// 该方法可以接收另一个 Rectangle 作为参数
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
