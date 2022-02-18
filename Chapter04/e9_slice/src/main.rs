// 字符串切片
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // 创建切片，索引为 0～4
    let world = &s[6..11]; // 创建切片，索引为 6～10

    // Rust 的范围语法 .. 的语法糖
    let len = s.len();

    let slice = &s[0..2];
    let slice = &s[..2];

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];
}
