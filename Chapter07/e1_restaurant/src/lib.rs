// 【例7-1】一个含有其他功能模块的 front_of_house 模块
mod front_of_house { // mod 定义模块
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 【例7-2】示例 7-1 中代码的树状模块结构
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// 【例7-3】分别使用绝对路径和相对路径来调用 add_to_waitlist 函数
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 【例7-4】构建示例 7-3 中的代码后产生的编译错误
// === Output ===
// error[E0603]: module `hosting` is private
//   --> src/lib.rs:32:28
//    |
// 32 |     crate::front_of_house::hosting::add_to_waitlist();
//    |                            ^^^^^^^ private module
//    |
// note: the module `hosting` is defined here
//   --> src/lib.rs:3:5
//    |
// 3  |     mod hosting {
//    |     ^^^^^^^^^^^

// error[E0603]: module `hosting` is private
//   --> src/lib.rs:35:21
//    |
// 35 |     front_of_house::hosting::add_to_waitlist();
//    |                     ^^^^^^^ private module
//    |
// note: the module `hosting` is defined here
//   --> src/lib.rs:3:5
//    |
// 3  |     mod hosting {
//    |     ^^^^^^^^^^^
