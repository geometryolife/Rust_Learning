// 【例6-4】Coin 枚举中的 Quarter 变体存放了一个 UsState 值
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --略--
}

enum Coin {
    Penney,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 假设有个朋友在尝试收集所有 50 个州的 25 美分硬币，在根据硬币类型
// 进行分类的时候，也可以打印出每个 25 美分硬币所对应的州的名字。
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penney => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    // === Output ===
    // State quarter from Alaska!
}
