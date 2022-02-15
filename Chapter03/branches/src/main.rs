fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//   Compiling branches v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter03/branches)
//    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
//     Running `/home/ubuntu/other/Rust_Learning/Chapter03/branches/target/debug/branches`
//number is divisible by 3
