fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 6 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

//the value is: 10
//the value is: 20
//the value is: 30
//the value is: 40
//the value is: 50
//thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/main.rs:6:38
//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
