// 程序员视角
// fn main() {
//     let integer = Some(5);
//     let float = Some(5.0);

//     println!("{:?}, {:?}", integer, float);
//     // === Output ===
//     // Some(5), Some(5.0)
// }

// 编译器做的单态化过程
#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    println!("{:?}, {:?}", integer, float);
    // === Output ===
    // Some(5), Some(5.0)
}
