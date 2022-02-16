fn main() {
    var_scope();
    string_type();
    memory_allocation();
    move_action();
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


// 内存与分配
// Rust 在作用域结束的地方自动调用 drop 函数 => C++ 中 RAII
fn memory_allocation() {
    let s = String::from("help"); // 从这里开始，变量 s 变得有效

    // 执行与 s 相关的操作
} // 作用域到这里结束，变量 s 失效


// 变量和数据交互的方式：移动（move）
// 示例4-2：将变量 x 绑定的整数值重新绑定到变量 y 上
fn move_action() {
    // 入栈
    let x = 5; // 绑定到 x
    let y = x; // 创建 x 拷贝，绑定到 y

    // 栈存储 + 堆分配
    // 指针（ptr）、长度（len）、容量（capacity）-> 堆空间
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);

    // === Output ===
    //error[E0382]: borrow of moved value: `s1`
    //  --> src/main.rs:50:28
    //   |
    //47 |     let s1 = String::from("hello");
    //   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    //48 |     let s2 = s1;
    //   |              -- value moved here
    //49 |
    //50 |     println!("{}, world!", s1);
    //   |                            ^^ value borrowed here after move
}


