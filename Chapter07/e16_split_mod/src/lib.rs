// 将模块拆分为不同的文件
mod front_of_house; // 会在当前文件路径下查找同名文件

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
