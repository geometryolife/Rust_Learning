// 假如我们想要在打印 25 美分硬币信息的同时，对处理过
// 的非 25 美分硬币进行计数。
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    let coin = Coin::Nickel;

    let mut count2 = 0;
    let coin2 = Coin::Quarter(UsState::Alabama);

    // match 表达式法
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("count: {}", count);

    // if let 表达式法
    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!", state);
    } else {
        count2 += 1;
    }
    println!("count2: {}", count2);
}
// === Output ===
// count: 1
// State quarter from Alabama!
// count2: 0
