fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

//   Compiling loops2 v0.1.0 (/home/ubuntu/other/Rust_Learning/Chapter03/loops2)
//    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
//     Running `/home/ubuntu/other/Rust_Learning/Chapter03/loops2/target/debug/loops2`
//3!
//2!
//1!
//LIFTOFF!!!
