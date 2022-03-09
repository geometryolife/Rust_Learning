// 【例9-10】只有在值位于 1～100 之间时才创建 Guess 实例
#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(10);
    println!("{:?}", guess);
    // === Output ===
    // Guess { value: 10 }

    let guess = Guess::new(101);
    println!("{:?}", guess);
    // === Output ===
    // thread 'main' panicked at 'Guess value must be between 1 and 100, got 101.', src/main.rs:10:13
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
