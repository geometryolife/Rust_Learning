use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argument");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("What text:\n{}", contents);

    Ok(())
}

// 【例12-15】基于我们对 search 函数行为的预期，创建一个暂时会失败的测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

// 【例12-16】定义一个恰好能让测试编译通过的 search 函数
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

// === Output ===
// running 1 test
// test tests::one_result ... FAILED

// failures:

// ---- tests::one_result stdout ----
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `["safe, fast, productive."]`,
//  right: `[]`', src/lib.rs:43:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


// failures:
//     tests::one_result

// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
