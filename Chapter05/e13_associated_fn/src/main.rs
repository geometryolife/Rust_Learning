// 关联函数
// 任务：编写一个接收一个维度参数的关联函数，它会将输入
// 的参数同时用作长度与宽度来构造正方形的 Rectangle 实例。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let sq = Rectangle::square(3);

    println!("{:?}", sq);
    // === Output ===
    // Rectangle { width: 3, height: 3 }
}

// 关联函数：不用接收 self 作为参数的函数
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
