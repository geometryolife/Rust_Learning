// 定义枚举
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // 使用枚举变体来创建实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 使用任一变体调用函数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    route(four);
    route(six);
}

// 定义一个接收 ApAddrKind 类型的函数
fn route(ip_type: IpAddrKind) {}

