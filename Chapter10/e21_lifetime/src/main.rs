// 使用生命周期来避免悬垂引用
// 【例10-17】尝试在值离开作用域时使用指向它的引用
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
    // === Output ===
    // error[E0597]: `x` does not live long enough
    //   --> src/main.rs:8:13
    //    |
    // 8  |         r = &x;
    //    |             ^^ borrowed value does not live long enough
    // 9  |     }
    //    |     - `x` dropped here while still borrowed
    // 10 |
    // 11 |     println!("r: {}", r);
    //    |                       - borrow later used here
}

// 【例10-18】r 与 x 的生命周期的标注，它们分别对应 'a 与 'b
// fn main() {
//     {
//         let r;                // ---------+-- 'a
//                               //          |
//         {                     //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;           //  |       |
//         }                     // -+       |
//                               //          |
//         println!("r: {}", r); //          |
//     }                         // ---------+
// }
