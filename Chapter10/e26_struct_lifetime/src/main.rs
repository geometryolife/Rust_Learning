// 【10-25】结构体中持有了引用，所以它的定义中需要添加生命周期标注
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i);
    // === Output ===
    // ImportantExcerpt { part: "Call me Ishmael" }
}
