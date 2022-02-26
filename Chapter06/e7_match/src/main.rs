// 【例6-3】一个枚举以及一个以枚举变体作为模式的 match 表达式
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = value_in_cents(Coin::Penny);
    println!("{}", coin);
    // === Output ===
    // Lucky penny!
    // 1
}
