#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 声明在 impl 及类型名称之后的生命周期是不能省略的，根据第一条省略规则，
// 可以不用为方法中的 self 引用标注生命周期
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 应用第三条生命周期省略规则
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let p = ImportantExcerpt { part: "Joe Chen" };
    println!("{:?}", p);
    // === Output ===
    // ImportantExcerpt { part: "Joe Chen" }

    let l = p.level();
    println!("{:?}", l);
    // === Output ===
    // 3

    let a = p.announce_and_return_part("Joe");
    println!("{:?}", a);
    // Attention please: Joe
    // "Joe Chen"
}
