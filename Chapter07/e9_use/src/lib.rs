// 【例7-13】使用 use 将 add_to_waitlist 函数引入作用域的非惯用方式
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist(); // 无法清晰地传达出函数的定义区域
    add_to_waitlist();
    add_to_waitlist();
}
