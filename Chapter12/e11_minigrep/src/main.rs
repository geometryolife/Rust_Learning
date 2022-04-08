// 【例12-14】将 minigrep 包引入 src/main.rs 的作用域中
use std::env;
use std::process;

use e11_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = e11_minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
