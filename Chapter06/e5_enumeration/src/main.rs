// 枚举方法
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    impl Message {
        fn call(&self) {
            // 方法体可以在这里定义
           println!("{:?}", self);
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
