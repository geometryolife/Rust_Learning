// 【例7-7】为 mod hosting 与 fn add_to_waitlist 添加的 pub 关键
// 字使我们可以在 eat_at_restaurant 中调用这一函数
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
