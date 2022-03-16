// 在枚举中定义泛型
fn main() {
    // 拥有一个泛型参数的枚举
    enum Option<T> {
        Some(T),
        None,
    }

    // 拥有两个泛型参数的枚举
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
