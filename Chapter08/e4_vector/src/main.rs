// 【例8-7】在存在指向动态数组元素的引用时尝试向动态数组中添加元素
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
    // === Output ===
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    //  --> src/main.rs:7:5
    //   |
    // 5 |     let first = &v[0];
    //   |                  - immutable borrow occurs here
    // 6 |
    // 7 |     v.push(6);
    //   |     ^^^^^^^^^ mutable borrow occurs here
    // 8 |
    // 9 |     println!("The first element is: {}", first);
    //   |                                          ----- immutable borrow later used here
}
