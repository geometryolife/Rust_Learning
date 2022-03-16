// 【例10-8】使用了两个泛型的 Point<T, U>，x 和 y 可以拥有不同类型的值
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("{:?}, {:?}, {:?}", both_integer, both_float, integer_and_float);
    // === Output ===
    // Point { x: 5, y: 10 }, Point { x: 1.0, y: 4.0 }, Point { x: 5, y: 4.0 }
}
