#[derive(Debug)]
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

    // 使用结构体更新语法根据其他实例创建新实例
    // 【例5-7】使用结构体更新语法来为一个 User 实例设置新的 email 和 username 字段的值，
    // 并从 user1 实例中获取剩余字段的值
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{:?}", user2);
    // === Output ===
    // User { username: "anotherusername567", email: "another@example.com", sign_in_count: 1, active: true }
}
