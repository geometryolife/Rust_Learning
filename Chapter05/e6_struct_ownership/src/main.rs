// 在没有生命周期的情形下，直接在结构体中声明引用字段
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active:bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

// === Output ===
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:2:15
//   |
// 2 |     username: &str,
//   |               ^ expected named lifetime parameter
//   |
// help: consider introducing a named lifetime parameter
//   |
// 1 | struct User<'a> {
// 2 |     username: &'a str,
//   |

// error[E0106]: missing lifetime specifier
//  --> src/main.rs:3:12
//   |
// 3 |     email: &str,
//   |            ^ expected named lifetime parameter
//   |
// help: consider introducing a named lifetime parameter
//   |
// 1 | struct User<'a> {
// 2 |     username: &str,
// 3 |     email: &'a str,
//   |

// error: aborting due to 2 previous errors

// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `e6_struct_ownership`

// To learn more, run the command again with --verbose.
