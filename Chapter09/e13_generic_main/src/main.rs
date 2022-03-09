use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
// === Output ===
// Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
