fn main() {
    // 【例8-15】使用 push_str 方法向 String 中添加字符串切片
    let mut s = String::from("foo");
    println!("s = {}", s);

    s.push_str("bar");

    println!("s = {}", s);
    // === Output ===
    // s = foo
    // s = foobar

    // 【例8-16】在将字符串切片附加至 String 后继续使用它
    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    println!("{}", s2);
    // === Output ===
    // bar

    // 【例8-17】使用 push 方法将一个字符添加到 String 中
    let mut s = String::from("lo");

    s.push('l');

    println!("s = {}", s);
    // === Output ===
    // s = lol

    // 【8-18】使用 + 运算符将两个 String 合并到一个新的 String 中
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意这里的 s1 已经被移动且再也不能被使用了

    // + 运算符会调用一个 add 方法，它的函数签名如下：
    // fn add(self, s: &str) -> String

    println!("{}", s3);
    // === Output ===
    // Hello, world!

    // 蹩脚的 + 运算符
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);
    // === Output ===
    // tic-tac-toe

    // 优雅的 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    // === Output ===
    // tic-tac-toe
}
