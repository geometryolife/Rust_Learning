fn main() {
    var_scope();
    string_type();
}

// 变量作用域
fn var_scope() { // 由于变量 s 还未被声明，所以它在这里是不可用的
    let s = "hello"; // 从这里开始变量 s 变得可用

    // 执行与 s 相关的操作
} // 作用域到这里结束，变量 s 再次不可用

// String 类型
// 在堆分配空间，处理编译时未知大小的文本
fn string_type() {
    let s = String::from("hello"); // from 函数根据字面量创建 String 实例

    let mut s2 = String::from("hello");

    s2.push_str(", world!"); // push_str 函数向 String 空间的尾部添加了一段字面量

    println!("{}", s2); // 这里会输出完整的 hello, world!
}
