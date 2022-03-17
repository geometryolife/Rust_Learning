// 【例10-16】根据泛型的 trait 约束来有条件地实现方法
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 类型 Pair<T> 都会实现 new 函数
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// 只有在内部类型 T 实现了 PartialOrd 和 Display 这
// 两个 trait 的前提下，才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p = Pair::new(5, 4);

    p.cmp_display();
    // === Output ===
    // The largest member is x = 5
}
