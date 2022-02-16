// 所有权与函数
// 示例4-3：函数中所有权和作用域的变化过程
fn main() {
    let s = String::from("hello"); // 变量 s 进入作用域

    takes_ownership(s); // s 的值被移动进了函数
    // 所以它从这里开始不再有效

    let x = 5; // 变量 x 进入作用域

    makes_copy(x); // 变量 x 同样被传递进了函数，
    // 但由于 i32 是 Copy 的，所以我们依然可以在这之后使用 x

    // println!("use s again: {}", s);
    // === Output ===
    // error[E0382]: borrow of moved value: `s`
    println!("use x again: {}", x);

} // x 首先离开作用域，随后是 s
// 但由于 s 的值已经发生了移动，所以没有什么特别的事情会发生

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // some_string 在这里离开作用域，drop 函数被自动调用，
// some_string 所占用的内存也就随之被释放了

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // some_integer 在这里离开了作用域，没有什么特别的事情发生

// === Output ===
// hello
// 5
// use x again: 5
