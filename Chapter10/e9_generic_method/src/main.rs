// 【例10-9】为结构体 Point<T> 实现名为 x 的方法，它会返回一个
// 指向 x 字段中 T 类型值的引用
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10};

    println!("p.x = {}", p.x());
    // === Output ===
    // p.x = 5
}
