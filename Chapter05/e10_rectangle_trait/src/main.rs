// 通过派生 trait 增加实用功能
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // Rust 没有为结构体提供默认的 Display 实现
    // println!("rect1 is {}", rect1);
    // === Output ===
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    //  --> src/main.rs:9:29
    //   |
    // 9 |     println!("rect1 is {}", rect1);
    //   |                             ^^^^^ `Rectangle` cannot be formatted with the default formatter
    //   |
    //   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    //   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //   = note: required by `std::fmt::Display::fmt`
    //   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

    // 添加了 {:?}，但没添加 derive 注解
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    //   --> src/main.rs:25:31
    //    |
    // 25 |     println!("rect1 is {:?}", rect1); // 使用 Debug 格式化输出
    //    |                               ^^^^^ `Rectangle` cannot be formatted using `{:?}`
    //    |
    //    = help: the trait `Debug` is not implemented for `Rectangle`
    //    = note: add `#[derive(Debug)]` or manually implement `Debug`
    //    = note: required by `std::fmt::Debug::fmt`
    //    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

    println!("rect1 is {:?}", rect1); // 使用 Debug 格式化输出
    // === Output ===
    // rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1);
    // === Output ===
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}
