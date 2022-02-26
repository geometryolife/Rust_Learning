// 如果不想处理所有可能的值，可以使用通配符 _ 模式
fn main() {
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 匹配所有没有被显式指定出来的可能情形
    }
}
