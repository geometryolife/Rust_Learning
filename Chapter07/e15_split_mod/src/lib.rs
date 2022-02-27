// 【例7-21】声明 front_of_house 模块，其代码位于 src/front_of_house.rs 文件中
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
