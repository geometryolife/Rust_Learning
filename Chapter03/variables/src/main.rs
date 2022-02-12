fn main() {
    // 初始化数组的方式
    let a = [1, 2, 3, 4, 5]; // 初始化数组

    let months = ["January", "February", "March", "April", "May", "June",
    "July", "August", "Septembe", "October", "November", "December"];

    let b: [i32; 5] = [1, 2, 3, 4, 5]; // 显式写出数组的类型

    // 创建一个含有相同元素的数组
    // 等价于 let c = [3, 3, 3, 3, 3];
    let c = [3; 5];
}
