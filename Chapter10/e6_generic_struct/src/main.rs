// 【例10-6】存储了 T 类型值 x 与 y 的 Point<T> 结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?}, {:?}", integer, float);
    // === Output ===
    // Point { x: 5, y: 10 }, Point { x: 1.0, y: 4.0 }

    // 【例10-7】字段 x 和 y 必须是相同的类型，因为它们拥有相同的泛型 T
    let wont_work = Point { x: 5, y: 4.0};
    // === Output ===
    // error[E0308]: mismatched types
    //   --> src/main.rs:17:38
    //    |
    // 17 |     let wont_work = Point { x: 5, y: 4.0};
    //    |                                      ^^^ expected integer, found floating-point number
}
