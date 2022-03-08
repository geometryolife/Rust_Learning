// 【例8-22】一旦键值对被插入，其所有权就会转移给哈希映射
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    // field_name 和 field_value 从这一刻开始失效，若尝试使用它们则会导致编译错误！
    // println!("{}", field_name);
    // === Output ===
    // error[E0382]: borrow of moved value: `field_name`
    //   --> src/main.rs:12:20
    //    |
    // 5  |     let field_name = String::from("Favorite color");
    //    |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
    // ...
    // 10 |     map.insert(field_name, field_value);
    //    |                ---------- value moved here
    // 11 |     // field_name 和 field_value 从这一刻开始失效，若尝试使用它们则会导致编译错误！
    // 12 |     println!("{}", field_name);
    //    |                    ^^^^^^^^^^ value borrowed here after move
}
