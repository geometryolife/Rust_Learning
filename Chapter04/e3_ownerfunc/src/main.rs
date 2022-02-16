// 返回值与作用域
// 示例4-4：函数在返回值时所有权的转移过程
fn main() {
    let s1 = gives_ownership(); // gives_ownership 将它的返回值移动至 s1 中

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动进函数
    // takes_and_gives_back 中，而这个函数的返回值又被移动到了变量 s3 上

} // s3 在这里离开作用域并被销毁。由于 s2 已经移动了，
// 所以它不会在离开作用域时发生任何事情。s1 最后离开作用域并被销毁。

fn gives_ownership() -> String { // gives_ownership 会将它的返回值移动至调用它的函数内
    let some_string = String::from("hello"); // some_string 进入作用域

    some_string // some_string 作为返回值移动至调用函数
}

// takes_and_gives_back 将取得一个 String 的所有权并将它作为结果返回
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // a_string 作为返回值移动至调用函数
}
