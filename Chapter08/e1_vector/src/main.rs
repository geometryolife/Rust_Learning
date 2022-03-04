fn main() {
    // 【例8-1】创建一个用来存储 i32 数据的空动态数组
    let v: Vec<i32> = Vec::new(); // Vec 的关联函数 new 创建一个空动态数组

    println!("{:?}", v);
    // === Output ===
    // []

    // 【例8-2】创建一个包含了值的新动态数组
    // 不需要使用类型标记
    let v = vec![1, 2, 3]; // vec! 宏根据提供的值创建动态数组

    println!("{:?}", v);
    // === Output ===
    // [1, 2, 3]

    // 【例8-3】使用 push 方法将值添加到动态数组中
    // Rust 可以从后面推进来的数据进行类型推断，所以不需要显式类型标记
    let mut v = Vec::new(); 

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);
    // === Output ===
    // [5, 6, 7, 8]

    // 【8-4】展示了动态数组及其元素销毁的地方
    {
        let v2 = vec![1, 2, 3, 4];

        // 执行与 v2 相关的操作
        println!("v2: {:?}", v2);
        // === Output ===
        // v2: [1, 2, 3, 4]

    } // v2 在这里离开作用域并随之被销毁

    // println!("v2: {:?}", v2);
    // === Output ===
    // error[E0425]: cannot find value `v2` in this scope
    //   --> src/main.rs:38:26
    //    |
    // 38 |     println!("v2: {:?}", v2);
    //    |                          ^^ help: a local variable with a similar name exists: `v`
}
