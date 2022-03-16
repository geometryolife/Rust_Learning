// 【例10-10】这里的 impl 代码块只作用于使用具体类型替换了泛型参数 T 的结构体
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0};

    println!("Distance from origin = {}", p.distance_from_origin());
    // === Output ===
    // Distance from origin = 5
}
