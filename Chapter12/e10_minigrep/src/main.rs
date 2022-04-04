use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // 在 main 中处理 run 函数返回的错误
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

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

// 【例12-12】修改 run 函数使其返回 Result
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("What text:\n{}", contents);

    Ok(())
}

// === Output ===
// Problem parsing arguments: not enough arguments

// cargo run a aa
// === Output ===
// Searching for a
// In file aa
// Application error: No such file or directory (os error 2)

// === Output ===
// Searching for the
// In file poem.txt
// What text:
// I'm nobody! Who are you?
// Are you nobody, too?
// Then there's a pair of us - don't tell!
// They'd banish us, you know.

// How dreary to be somebody!
// How public, like a frog
// To tell your name the livelong day
// To an admiring bog!
