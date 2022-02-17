//fn main() {
//    let s = String::from("hello");

//    change(&s);
//}

//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

// === Output ===
//error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// --> src/main.rs:8:5
//  |
//7 | fn change(some_string: &String) {
//  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
//8 |     some_string.push_str(", world");
//  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

//error: aborting due to previous error


// 可变引用
// 对于特定作用域中的特定数据来说，一次只能声明一个可变引用：避免数据竞争
//fn main() {
//    let mut s = String::from("hello");

//    let r1 = &mut s;
//    let r2 = &mut s;

//    println!("{}, {}", r1, r2);
//}

// === Output ===
//error[E0499]: cannot borrow `s` as mutable more than once at a time
//  --> src/main.rs:28:14
//   |
//27 |     let r1 = &mut s;
//   |              ------ first mutable borrow occurs here
//28 |     let r2 = &mut s;
//   |              ^^^^^^ second mutable borrow occurs here
//29 |
//30 |     println!("{}, {}", r1, r2);
//   |                        -- first borrow later used here

//error: aborting due to previous error


 // 利用花括号创建多个可变引用
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // 由于 r1 在这里离开了作用域，所以我们可以合法地再创建一个可变引用

    let r2 = &mut s;
}


// 不能在拥有不可变引用的同时创建可变引用
//fn main() {
    //let mut s = String::from("hello");

    //let r1 = &s; // 没问题
    //let r2 = &s; // 没问题
    //let r3 = &mut s; // 错误！

    //println!("{}, {}, and {}", r1, r2, r3);
//}

// === Output ===
//error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
//  --> src/main.rs:66:14
//   |
//64 |     let r1 = &s; // 没问题
//   |              -- immutable borrow occurs here
//65 |     let r2 = &s; // 没问题
//66 |     let r3 = &mut s; // 错误！
//   |              ^^^^^^ mutable borrow occurs here
//67 |
//68 |     println!("{}, {}, and {}", r1, r2, r3);
//   |                                -- immutable borrow later used here

//error: aborting due to previous error
