// 直接将关联的数据嵌入枚举变体中
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    // 比【例6-1】利用结构体和枚举组合的方式更简捷
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

// Rust 标准库内置的实现
// struct Ipv4Addr {
//     // ---略---
// }

// struct Ipv6Addr {
//     // ---略---
// }

// enum IpAddr {
//     V4(IpV4Addr),
//     V6(IpV6Addr),
// }
