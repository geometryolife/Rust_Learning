// 一个使用结构体的示例程序
// 【例5-8】分别指定宽度和高度变量来计算长方形的面积
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width1: u32, height1: u32) -> u32 {
    width1 * height1
}

// === Output ===
// The area of the rectangle is 1500 square pixels.
