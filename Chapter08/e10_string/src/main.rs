fn main() {
    // 【例8-19】尝试对字符串使用索引语法
    // let s1 = String::from("hello");
    // let h = s1[0];
    // === Output ===
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    //  --> src/main.rs:4:13
    //   |
    // 4 |     let h = s1[0];
    //   |             ^^^^^ `String` cannot be indexed by `{integer}`
    //   |
    //   = help: the trait `Index<{integer}>` is not implemented for `String`

    // 内部布局
    // String 实际上是一个基于 Vec<u8> 的封装类型
    let len = String::from("Hola").len();
    println!("{}", len);
    // === Output ===
    // 4

    let len = String::from("Здравствуйте").len();
    println!("{}", len);
    // === Output ===
    // 24

    // let hello = "Здравствуйте";
    // let answer = &hello[0]; // 非法！
    // === Output ===
    // error[E0277]: the type `str` cannot be indexed by `{integer}`
    //   --> src/main.rs:27:19
    //    |
    // 27 |     let answer = &hello[0]; // 非法！
    //    |                   ^^^^^^^^ string indices are ranges of `usize`
    //    |
    //    = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
    //    = note: you can use `.chars().nth()` or `.bytes().nth()`
    //            for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
    //    = note: required because of the requirements on the impl of `Index<{integer}>` for `str`

    // 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("{}", s);
    // === Output ===
    // Зд

    // let s = &hello[0..1];
    // println!("{}", s);
    // === Output ===
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:48:14
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // 遍历字符串的方法
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // === Output ===
    // न
    // म
    // स
    // ्
    // त
    // े

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // === Output ===
    // 224
    // 164
    // 168
    // 224
    // 164
    // 174
    // 224
    // 164
    // 184
    // 224
    // 165
    // 141
    // 224
    // 164
    // 164
    // 224
    // 165
    // 135
}
