// 【例7-5】将 hosting 模块标记为 pub 以便在 eat_at_restaurant 中使用它
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 【例7-6】构建示例 7-5 中的代码后产生的编译错误
// === Output ===
// error[E0603]: function `add_to_waitlist` is private
//   --> src/lib.rs:10:37
//    |
// 10 |     crate::front_of_house::hosting::add_to_waitlist();
//    |                                     ^^^^^^^^^^^^^^^ private function
//    |
// note: the function `add_to_waitlist` is defined here
//   --> src/lib.rs:4:9
//    |
// 4  |         fn add_to_waitlist() {}
//    |         ^^^^^^^^^^^^^^^^^^^^

// error[E0603]: function `add_to_waitlist` is private
//   --> src/lib.rs:13:30
//    |
// 13 |     front_of_house::hosting::add_to_waitlist();
//    |                              ^^^^^^^^^^^^^^^ private function
//    |
// note: the function `add_to_waitlist` is defined here
//   --> src/lib.rs:4:9
//    |
// 4  |         fn add_to_waitlist() {}
//    |         ^^^^^^^^^^^^^^^^^^^^
