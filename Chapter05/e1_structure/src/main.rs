// 定义并实例化结构体
// 【例5-1】User 结构体的定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    // 【例5-2】创建一个 User 结构体的实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }; // 不要忘记分号

    // 使用点号访问实例的特定字段
    println!("Name: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign_in_count: {}", user1.sign_in_count);
}

// === Output ===
// Name: someusername123
// Email: someone@example.com
// Active: true
// Sign_in_count: 1
