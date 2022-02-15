fn main() {
    let a = [10, 20, 30, 40, 50];


    // 不需要手动指定终止索引，比 while 循环遍历更安全
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

//The value is: 10
//The value is: 20
//The value is: 30
//The value is: 40
//The value is: 50
