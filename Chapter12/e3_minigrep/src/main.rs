// 【例12-4】读取第二个参数所指定文件中的内容
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// === Output ===
// Searching for the
// In file poem.txt
// With text:
// I'm nobody! Who are you?
// Are you nobody, too?
// Then there's a pair of us - don't tell!
// They'd banish us, you know.

// How dreary to be somebody!
// How public, like a frog
// To tell your name the livelong day
// To an admiring bog!
