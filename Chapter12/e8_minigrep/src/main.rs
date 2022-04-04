use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

// 【例12-10】在创建 Config 实例失败时使用错误码退出程序
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // 程序退出时，向调用者（父进程）返回非 0 的状态码是一种惯用的信号，
        // 它表明当前程序的退出是由某种错误状态导致的。
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// 【12-9】让 Config::new 返回一个 Result
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// === Output ===
// Problem parsing arguments: not enough arguments
