// 定义并实例化结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 【例5-3】修改 User 实例中 email 字段的值
    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);
    // === Output ===
    // someone@example.com
    // anotheremail@example.com
}

// 【例5-4】一个接收邮箱和用户名作为参数并返回 User 实例的函数 build_user
// 函数参数与结构体字段名相同让代码更易读
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 在变量名与字段名相同时，使用简化版的字段初始化方法
// 【例5-5】build_user2 函数中使用了相同的参数名与字段名，并采用了字段初始化简写语法进行编写
// build_user2 是 build_user 的简化版，但功能并未改变
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
