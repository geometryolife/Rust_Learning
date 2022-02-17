// 悬垂引用
fn main() {
    // let reference_to_nothing = dangle();
    let reference_to_something = safe();

    println!("{}", reference_to_something);
}

//fn dangle() -> &String { // dangle 会返回一个指向 String 的引用
//    let s = String::from("hello"); // s 被绑定到新的 String 上

//    &s // 我们将指向 s 的引用返回给调用者
//} // 变量 s 在这里离开作用域并随之被销毁，它指向的内存自然也不再有效。危险！

// === Output ===
//error[E0106]: missing lifetime specifier
// --> src/main.rs:6:16
//  |
//6 | fn dangle() -> &String {
//  |                ^ expected named lifetime parameter
//  |
//  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//help: consider using the `'static` lifetime
//  |
//6 | fn dangle() -> &'static String {
//  |                ^^^^^^^^


fn safe() -> String {
    let s = String::from("hello");

    s // 将所有权转出给调用函数
}
