// 【例8-10】在动态数组中使用定义的枚举来存储不同类型的值
fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
    // === Output ===
    // [Int(3), Text("blue"), Float(10.12)]
}
