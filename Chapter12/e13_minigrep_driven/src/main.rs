use std::env;
use std::process;

use e13_minigrep_driven::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err);
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = e13_minigrep_driven::run(config) {
        // println!("Application error: {}", e);
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// cargo run > output.txt
// === Output ===
// Problem parsing arguments: not enough argument

// 正常情况
// cargo run to poem.txt > output.txt
