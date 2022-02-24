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

    // 不使用结构体更新语法根据其他实例创建新实例
    // 【例5-6】使用 user1 中的某些值来创建一个新的 User 实例
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("{:?}", user2);
    // === Output ===
    // User { username: "anotherusername567", email: "another@example.com", sign_in_count: 1, active: true }
}
