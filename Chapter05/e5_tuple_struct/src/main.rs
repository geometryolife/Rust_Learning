// 元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);
    // === Output ===
    // Color(0, 0, 0)
    // Point(0, 0, 0)

    // 通过模式匹配解构来访问元组结构（体）特定的字段
    // let (x, y, z) = origin;
    // println!("x: {}, y: {}, z: {}", x, y, z);
    // TODO: 暂时还没成功！

    // 通过点号和索引来访问元组结构的特定字段
    println!("{}", origin.2);
    // === Output ===
    // 0
}
