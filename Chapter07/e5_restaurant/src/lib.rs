// 早餐模型：客户可以自行选择想要的面包，但只有
// 厨师才能根据季节与存货决定配餐水果。
// 【例7-9】；一个拥有部分公共字段、部分私有字段的结构体
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 选择黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // 修改我们想要的面包类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 接下来的这一行无法通过编译，我们不能看到或更换随着食物附带的季节性水果
    // meal.seasonal_fruit = String::from("blueberries");
    // === Output ===
    // error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
    //   --> src/lib.rs:29:10
    //    |
    // 29 |     meal.seasonal_fruit = String::from("blueberries");
    //    |          ^^^^^^^^^^^^^^ private field
}
