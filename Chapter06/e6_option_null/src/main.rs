// Option 枚举及在空值处理方面的优势
// Rust 没有空值，但提供了拥有类似概念的枚举：
// enum Option<T> {
//     Some(T),
//     None,
// }
// Option<T> 包含在预导入模块中
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // 使用 None 变体来赋值，需要明确告诉 Rust 这个 Option<T> 的具体类型
    let absent_number:Option<i32> = None;

    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
    // === Output ===
    // Some(5), Some("a string"), None

    // i8 和 Option<i8> 类型不同，无法相加
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
    // === Output ===
    // error[E0277]: cannot add `Option<i8>` to `i8`
    //   --> src/main.rs:22:17
    //    |
    // 22 |     let sum = x + y;
    //    |                 ^ no implementation for `i8 + Option<i8>`
    //    |
    //    = help: the trait `Add<Option<i8>>` is not implemented for `i8`
}
